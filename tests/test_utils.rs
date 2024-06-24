use std::path::Path;

use esign_rust_api::tools::utils::{calculate_file_md5, decode, encode, encode_by_file_async, save_file_base64};


#[test]
fn test_encode() {
    let str = "123456";
    let encode = encode(str);
    println!("{:?}",encode)       

}
#[test]
fn test_decode() {
    let str = "MTIzNDU2";
    let decode =decode (str).unwrap();
    println!("{:?}",String::from_utf8(decode))       

}
#[tokio::test]
async fn test_encode_by_file_async() {
    let str = Path::new("./src/files/test.txt");
    let file = encode_by_file_async(str).await.unwrap();
    println!("{:?}",file)       

}
#[tokio::test]
async fn test_save_file_base64() {
    let str = "aGVsbG8gd29ybGQ=";
    let path = "./src/files/test1.txt";
    println!("{:?}",save_file_base64(path,str).await)
   
}
#[test]
fn test_hash_md5() {
    let str = "123456";
    let hash = esign_rust_api::tools::utils::hash_md5(str);
    println!("{:?}",hash)       

}
#[tokio::test]
async fn test_calculate_file_md5() {
    let path = "./src/files/test1.txt";
    println!("{:?}",calculate_file_md5(path).await)       

}
