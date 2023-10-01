
use std::time::SystemTime;
use std::collections::HashMap;

use crate::rocket::futures::StreamExt;

use wasmflow_sdk::v1::transport::{MessageTransport, TransportStream};
use wasmflow_sdk::v1::transport::TransportMap;
use wasmflow_sdk::v1::{Entity, InherentData, Invocation};
use wasmflow_rpc::error::RpcClientError;

pub async fn invoke(address: &str,port:&str,component:&str, inputs: TransportMap,seed: Option<u64>) -> Result<HashMap<String,MessageTransport>,RpcClientError> {
    //let _guard = crate::utils::init_logger(&opts.logging)?;
  
    let mut client = wasmflow_rpc::make_rpc_client(
      format!("http://{}:{}", address, port),
      None,None,None,None)
    .await?;
  
    let origin = Entity::client("rstreamer");
    let target = Entity::local(component);
  
    let inherent_data = seed.map(|seed| {
      InherentData::new(
        seed,
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .unwrap()
          .as_millis()
          .try_into()
          .unwrap(),
      )
    });


    let invocation = Invocation::new(origin, target, inputs, inherent_data);
    trace!("issuing invocation");
    let mut stream = client.invoke(invocation).await?;
    trace!("server responsed");
    
  
    let mut result = HashMap::new();

    while let Some(wrapper) = stream.next().await {
  
      if wrapper.payload.is_signal(){
        continue;
      }
  
      result.insert(wrapper.port,wrapper.payload);
      
    }
  
    Ok(result)
  }



pub async fn print_stream_json(mut stream: TransportStream) -> String {

  let mut result = String::from("");

  while let Some(wrapper) = stream.next().await {

    if wrapper.payload.is_signal(){
      continue;
    }

    result += &wrapper.as_json().to_string()
    
  }

  result 
}