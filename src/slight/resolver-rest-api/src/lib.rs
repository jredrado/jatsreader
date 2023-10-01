use anyhow::Result;

use http_server::*;
use slight_http_handler_macro::register_handler;
use slight_http_server_macro::on_server_init;

wit_bindgen_rust::import!("../wit/http-server.wit");
wit_bindgen_rust::export!("../wit/http-server-export.wit");
wit_bindgen_rust::import!("../wit/http-client.wit");
wit_error_rs::impl_error!(http_server::HttpRouterError);
wit_error_rs::impl_error!(http_client::HttpError);

use configs::*;
wit_bindgen_rust::import!("../wit/configs.wit");
wit_error_rs::impl_error!(ConfigsError);


use resolver::{ResolverClient,StreamerInfo};

use serde_json_lenient;

use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

static mut CONFIG_INSTANCE: Option<Mutex<String>> = None;
static INIT: Once = Once::new();

fn resolver_instance<'a>() -> &'a Mutex<String> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        if let Ok(configs) = Configs::open("config-store") {
            if let Ok(r) = configs.get(&"RESOLVERINSTANCE") {
                if let Ok(resolver_instance) = String::from_utf8(r){
                    unsafe {
                        *CONFIG_INSTANCE.borrow_mut() = Some(Mutex::new(resolver_instance));
                    }
                }
            }
        }
    });

    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { CONFIG_INSTANCE.as_ref().unwrap() }
}

#[on_server_init]
fn main() -> Result<()> {


    let router = Router::new()?;
    let router_with_route = router
        .get("/pub/:id/resolve","handle_resolver")?;

    println!("Starting server");
    let _ = Server::serve("0.0.0.0:3001", &router_with_route)?;
    // server.stop().unwrap();
    println!("Serving...");

    Ok(())
}

#[register_handler]
fn handle_resolver(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);

    let mut headers = request.headers.clone();  
    headers.push((String::from("Content-Type"),String::from("application/json")));
    

    let client = ResolverClient::new(&*resolver_instance().lock().unwrap()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let streamers = client.get_streamers(id.1.to_owned()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let manifests : Vec<StreamerInfo> = streamers.iter().map(|s| StreamerInfo{id:s.id.clone(),endpoint:format!("{}/pub/{}/manifest.json",&s.endpoint,&id.1)}).collect();

    let json_response = serde_json_lenient::to_string(&manifests).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    Ok(Response {
        headers: Some(headers),
        body: Some(json_response.as_bytes().to_vec()),
        status: 200,
    })
}
