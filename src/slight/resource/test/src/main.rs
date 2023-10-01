use anyhow::Result;

use resource::ResourceClient;
use register::RegisterClient;

fn main() -> Result<()> {

     
    let client = RegisterClient::new("register_1")?;

    let epubbytes = include_bytes!("../../../../../assets/9123624.epub");

    let id = client.register(epubbytes.to_vec())?;

    println!("ID: {}",id);

    
    let rclient = ResourceClient::new("resource_1")?;
    let resource = rclient.resource(id,b"EPUB/9123624.xml".to_vec())?;

    println!("Resource: \n {:?}",std::str::from_utf8(&resource.1));

    Ok(())
}
