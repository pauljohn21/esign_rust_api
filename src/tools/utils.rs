use anyhow::{Context, Ok, Result};
use base64::prelude::*;
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use reqwest::Method;
use sha2::Sha256;
use std::io::Write;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
type HmacSha256 = Hmac<Sha256>;

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    BASE64_STANDARD.encode(input.as_ref())
}
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    BASE64_STANDARD.decode(input.as_ref()).context("解码失败")
}

pub async fn encode_by_file_async<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let canonical_path = fs::canonicalize(file_path.as_ref())
        .await
        .context("路径不合法")?;
    let mut input = File::open(&canonical_path).await.context("读取文件失败")?;
    let mut buffer = Vec::new();
    input
        .read_to_end(&mut buffer)
        .await
        .context("读取文件失败")?;
    Ok(encode(buffer))
}
pub async fn save_file_base64<P: AsRef<Path>>(file_path: P, base64_str: &str) -> Result<()> {
    let decode_data = decode(base64_str).context("解码失败")?;
    let mut output = File::create(file_path).await.context("创建文件失败")?;
    output
        .write_all(&decode_data)
        .await
        .context("写入文件失败")?;
    Ok(())
}
pub fn hash_md5(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    encode(result)
}
pub async fn calculate_file_md5<P: AsRef<Path>>(file_path: P) -> Result<(String, u64)> {
    let mut file = File::open(file_path).await.context("打开文件失败")?;
    let file_size = file.metadata().await.context("获取文件元数据失败")?.len();
    let file_chunk = 8192;
    let blocks = ((file_size as f64) / (file_chunk as f64)).ceil() as u64;
    let mut hash = Md5::new();
    for block in 0..blocks {
        let block_start = block * file_chunk as u64;
        let block_size = if block_start + file_chunk as u64 > file_size {
            file_size - block_start
        } else {
            file_chunk as u64
        };
        file.seek(std::io::SeekFrom::Start(block_start))
            .await
            .context(format!("跳转到文件块位置{}时出错", block_start))?;
        let mut buf = vec![0; block_size as usize];
        file.read_exact(&mut buf)
            .await
            .context(format!("读取文件块{}时出错", block))?;
        hash.update(&buf);
    }
    let digest = hash.finalize();
    let encode_hash = encode(digest);
    Ok((encode_hash, file_size))
}

pub fn signature_base64(msg: &str, secret: &str) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_ref()).context("生成签名失败")?;
    mac.update(msg.as_ref());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(encode(code_bytes))
}

pub fn append_sign_data_string(
    method: Method,
    accept: &str,
    content_md5: &str,
    content_type: &str,
    date: &str,
    headers: &str,
    url: &str,
) -> Result<String> {
    let mut buffer = Vec::new();

    writeln!(buffer, "{}", method)?;
    writeln!(buffer, "{}", accept)?;
    writeln!(buffer, "{}", content_md5)?;

    writeln!(buffer, "{}", content_type)?;
    writeln!(buffer, "{}", date)?;

    if headers.is_empty() {
        write!(buffer, "{}{}", headers, url)?;
    } else {
        writeln!(buffer, "{}", headers)?;
        writeln!(buffer, "{}", url).unwrap();
    }

    Ok(String::from_utf8(buffer)?)
}
