use anyhow::Result;

use register::RegisterClient;

fn main() -> Result<()> {

    let client = RegisterClient::new("register_1")?;

    let epubbytes = include_bytes!("../../../../../assets/9123624.epub");

    let id = client.register(epubbytes.to_vec())?;

    println!("ID: {}",id);

    let epubbytes_2 = include_bytes!("../../../../../assets/256071.epub");

    let id2 = client.register(epubbytes_2.to_vec())?;

    println!("ID2: {}",id2);

    

    Ok(())
}
