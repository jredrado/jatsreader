use anyhow::Result;

use locateverifier::LocateVerifierClient;
use register::RegisterClient;

fn main() -> Result<()> {

     
    let client = RegisterClient::new("register_1")?;

    let epubbytes = include_bytes!("../../../../../assets/9123624.epub");

    let id = client.register_with(epubbytes.to_vec(),"storage_1".to_string())?;

    println!("ID: {}",id);

    
    let lclient = LocateVerifierClient::new("locateverifier_1")?;

    /*
    {
        "href" : "/EPUB/9123624.xml",
        "media_type": "text/xml",
        "from_css_selector": "article body sec[0] p[0]",
        "to_css_selector": "article body sec[0] p[2]"
    }
    */

    let locate = lclient.locate_with(id,"EPUB/9123624.xml".to_string(),"text/xml".to_string(),
                            "article body sec p[0]".to_string(), 
                            "article body sec[0] p[2]".to_string(),
                            "locate_1".to_string(),
                            "storage_1".to_string()
    )?;

    println!("Locate : \n {:?}",&locate);

    Ok(())
}
