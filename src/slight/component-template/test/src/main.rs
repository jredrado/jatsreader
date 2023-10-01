use anyhow::Result;

use manifest::ManifestClient;
use register::RegisterClient;

fn main() -> Result<()> {

     
    let client = RegisterClient::new("register_1")?;

    let epubbytes = include_bytes!("../../../../../assets/9123624.epub");

    let id = client.register(epubbytes.to_vec())?;

    println!("ID: {}",id);

    
    let mclient = ManifestClient::new("manifest_1")?;
    let manifest = mclient.manifest(id)?;

    println!("Manifest: \n {:?}",manifest);

    Ok(())
}
