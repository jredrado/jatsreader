use anyhow::Result;

use storage::StorageClient;

fn main() -> Result<()> {

    let client = StorageClient::new("storage_1").unwrap();
    client.put("first1".to_string(),b"This is a test".to_vec());
    let value = client.get("first1".to_string()).unwrap();
    assert!( value == b"This is a test");

    let epubbytes = include_bytes!("../../../../../assets/9123624.epub");

    client.put("epub1".to_string(),epubbytes.to_vec());
    let epubvalue = client.get("epub1".to_string()).unwrap();
    assert!( epubvalue == epubbytes);

    Ok(())
}
