use anyhow::Result;
use resolver::{ResolverClient,StreamerInfo};

//Providers for an publication ID

pub struct Policy {

    //Register provider
    pub register_provider : String,

    //Service providers
    pub metadata_service : String,
    pub manifest_service : String,
    pub resource_service : String,
    pub locator_service : String,

    //Verifiers service providers
    pub metadata_verifier_service : String,
    pub manifest_verifier_service : String,
    pub resource_verifier_service : String,
    pub locator_verifier_service : String,

    //Storage providers
    pub storage_provider: String,

    //Resolver providers
    pub resolver_provider: String

}


impl Policy {

    pub fn new (id: &String) -> Result<Self> {

        let client = ResolverClient::new("resolver_1").map_err( |e| anyhow::Error::msg(e.to_string()))?;

        let streamers = client.get_streamers(id.to_owned()).map_err( |e| anyhow::Error::msg(e.to_string()))?;

        let first_storage = streamers.get(0).ok_or( anyhow::Error::msg("Unable to get first endpoint"))?;
        //let manifests : Vec<StreamerInfo> = streamers.iter().map(|s| StreamerInfo{id:s.id.clone(),endpoint:format!("{}/pub/{}/manifest.json",&s.endpoint,&id.1)}).collect();

        println!("First storage for {} {}",id,&first_storage.endpoint);

        Ok(Policy {
            register_provider : String::from("register_1"),
            metadata_service : String::from("metadata_1"),
            manifest_service : String::from("manifest_1"),
            resource_service : String::from("resource_1"),
            locator_service  : String::from("locator_1"),

            metadata_verifier_service : String::from("metadataverifier_1"),
            manifest_verifier_service : String::from("manifestverifier_1"),
            resource_verifier_service : String::from("resourceverifier_1"),
            locator_verifier_service  : String::from("locatorverifier_1"),

            resolver_provider  : String::from("resolver_1"),
            storage_provider : first_storage.endpoint.to_owned()
        })
    }

}

impl Default for Policy {

    fn default() -> Self{
        Policy {
                register_provider : String::from("register_1"),
                metadata_service : String::from("metadata_1"),
                manifest_service : String::from("manifest_1"),
                resource_service : String::from("resource_1"),
                locator_service  : String::from("locator_1"),

                metadata_verifier_service : String::from("metadataverifier_1"),
                manifest_verifier_service : String::from("manifestverifier_1"),
                resource_verifier_service : String::from("resourceverifier_1"),
                locator_verifier_service  : String::from("locatorverifier_1"),

                resolver_provider  : String::from("resolver_1"),
                storage_provider : String::from("")
        }
    }
}