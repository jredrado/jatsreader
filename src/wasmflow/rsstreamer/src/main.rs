#![feature(decl_macro)]

#[macro_use] extern crate rocket;

use wasmflow_sdk::v1::transport::TransportMap;
use wasmflow_sdk::v1::transport::MessageTransport;

use rocket::http::{Status, ContentType};
use rocket::response::content;
use rocket::response::status;
use rocket::response::Redirect;

use rocket::State;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use rocket::fs::FileServer;

use std::path::PathBuf;

use urlencoding::decode;

use authselect::SimplifiedLocator;

mod rpc;

struct Config<'a> {
    pub rpc_port : &'a str,
    pub rpc_host : &'a str
}

impl<'a> Config<'a> {
    pub fn new (host: &'a str, port : &'a str) -> Self {
        Config {
            rpc_host : host,
            rpc_port : port
        }
    }
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, rsstreamer!"
}

#[get("/<id>/manifest.json")]
async fn manifest(id: String,config: &State<Config<'_>>) ->  Result<(ContentType,content::RawJson<String>),status::Custom<String>> {


    let mut input_data_map = TransportMap::default();
    input_data_map.insert("id", MessageTransport::success(&id));

    let output_data_map = rpc::invoke(config.rpc_host,config.rpc_port,"epubmanifest",input_data_map,None).await.map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    if let Some(contenttype) = output_data_map.get("contenttype"){
        if let Some(data) = output_data_map.get("data"){
            let ct = contenttype.to_owned().deserialize::<String>().map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;
            let custom = ContentType::parse_flexible(&ct).ok_or(status::Custom(Status::InternalServerError, String::from("Wrong content type")))?;
            let sdata = data.to_owned().deserialize::<String>().map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

            Ok((custom, content::RawJson(sdata) ))
        }else {
            Err(status::Custom(Status::InternalServerError, String::from("No content type") ))
        }

    }else{
        Err(status::Custom(Status::InternalServerError, String::from("No data") ))
    }
    

}

#[get("/<id>/<path..>")]
async fn resource(id: String,path: PathBuf,config: &State<Config<'_>>) ->  Result<(ContentType,Vec<u8>),status::Custom<String>> {
    let mut input_data_map = TransportMap::default();
    input_data_map.insert("id", MessageTransport::success(&id));

    let path_string = path.to_str().ok_or(status::Custom(Status::InternalServerError, String::from("Wrong path")))?;

    input_data_map.insert("path", MessageTransport::success(&String::from(path_string)));

    let output_data_map = rpc::invoke(config.rpc_host,config.rpc_port,"epubresource",input_data_map,None).await.map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    if let Some(contenttype) = output_data_map.get("contenttype"){
        if let Some(data) = output_data_map.get("data"){
            let ct = contenttype.to_owned().deserialize::<String>().map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;
            let custom = ContentType::parse_flexible(&ct).ok_or(status::Custom(Status::InternalServerError, String::from("Wrong content type")))?;
            let sdata = data.to_owned().deserialize::<Vec<u8>>().map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

            Ok((custom, sdata ))
        }else {
            Err(status::Custom(Status::InternalServerError, String::from("No content type") ))
        }

    }else{
        Err(status::Custom(Status::InternalServerError, String::from("No data") ))
    }
}

#[get("/urs?<urs>")]
fn urs_redirect(urs: &str) -> Result<Redirect,status::Custom<String>> {
    
    if let Ok(decoded) = decode(urs) {
        let components : Vec<&str> = decoded.split("://").collect();
        
        let r_locator : Vec<&str> = components[1].split("/").collect();

        Ok(Redirect::to(uri!(locate(r_locator[0],r_locator[1]))))

    }else {
        Err(status::Custom(Status::InternalServerError, String::from("Wrong urs format") ))
    }
}

#[get("/locate/<id>/<payload>")]
async fn locate(id:String,payload:String,config: &State<Config<'_>>)->  Result<(ContentType,String),status::Custom<String>> {
    let decoded_payload = hex::decode(payload).map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    let locator: SimplifiedLocator = serde_json::from_slice(&decoded_payload).map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    let mut input_data_map = TransportMap::default();

    input_data_map.insert("id", MessageTransport::success(&id));
    input_data_map.insert("href", MessageTransport::success(&locator.href));
    input_data_map.insert("mediatype", MessageTransport::success(&locator.media_type));
    input_data_map.insert("from", MessageTransport::success(&locator.from_css_selector));
    input_data_map.insert("to", MessageTransport::success(&locator.to_css_selector));

    let output_data_map = rpc::invoke(config.rpc_host,config.rpc_port,"epublocator",input_data_map,None).await.map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    let output = output_data_map.get("output").ok_or(status::Custom(Status::InternalServerError, String::from("Wrong output")))?;

    let soutput = output.to_owned().deserialize::<String>().map_err( |e| status::Custom(Status::InternalServerError, e.to_string() ))?;

    Ok((ContentType::Plain,soutput))

    
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    if let Ok(cwd) = std::env::current_dir() {
        if let Some(path_string) = cwd.to_str() {

            let static_dir = format!("{}{}static", path_string,std::path::MAIN_SEPARATOR);

            let _rocket = rocket::build()
                .manage(Config::new("127.0.0.1","8060"))
                .attach(CORS)
                .mount("/", routes![index,urs_redirect,locate])
                .mount("/", FileServer::from(static_dir))
                .mount("/pub", routes![manifest,resource])             
                .launch()
                .await?;
        }
    }

    Ok(())
    
}