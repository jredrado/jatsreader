use anyhow::Result;

use resolver::{ResolverClient,StreamerInfo};

fn main() -> Result<()> {

     
    let client = ResolverClient::new("resolver_1")?;
    let cids = vec!["dadfds".to_string(),"dafdsfddd".to_string(),"dadadfadsf".to_string()];

    client.add_content(StreamerInfo{ id: "streamer_1".to_string(),endpoint: String::from("https://dadfdf.tech")},cids)?;
    
    let cid = client.get_streamers("dadfds".to_string())?;

    assert_eq!(cid.get(0).unwrap().id,"streamer_1".to_string());

    /* 
    let cids = vec!["dadfds".to_string(),"dafdsfddd".to_string(),"dadadfadsf".to_string()];
    client.remove_content("streamer_1".to_string(),cids)?;

    let streamers = client.get_streamers("dadfds".to_string())?;

    assert_eq!(streamers,Vec::<String>::new());
    */

    Ok(())
}
