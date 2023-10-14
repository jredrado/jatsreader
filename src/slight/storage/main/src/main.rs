use anyhow::Result;

mod types;
use types::*;

use keyvalue::*;
wit_bindgen_rust::import!("../../wit/keyvalue.wit");
wit_error_rs::impl_error!(KeyvalueError);

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

use configs::*;
wit_bindgen_rust::import!("../../wit/configs.wit");
wit_error_rs::impl_error!(ConfigsError);

fn main() -> Result<()> {

    let configs = Configs::open("config-store")?;

    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;


    let my_kv = Keyvalue::open(&instance)?;

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::Get(ref key) => {
                let raw_response = my_kv.get(key)?;
                Response::Get(raw_response)
            },
            Request::Put(ref key,ref value) => { 
                my_kv.set(key, value)?;
                Response::Put
            },
            Request::List => {
                let raw_response = my_kv.keys()?;
                Response::List(raw_response)
            }
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
