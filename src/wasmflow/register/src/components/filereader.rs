
pub use crate::components::generated::filereader::*;

use wasmflow_sdk::v1::packet::v1::Packet as V1;
use wasmflow_sdk::v1::packet::{Packet,PacketWrapper};

use tracing::{debug,info};

use std::{
  fs::File,
  io::{self, BufRead, BufReader},
};


#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

      //debug!("Init");

      let contents = std::fs::read(&inputs.filename).map_err(|e| format!("Could not read file {}: {}", inputs.filename, e))?;
      //let port = outputs.contents.get_port()?;
      //let name = outputs.contents.get_port_name();

      /* 
      const CAP: usize = 1024 * 1024;
      let file = File::open(&inputs.filename).map_err(|e| format!("Could not read file {}: {}", inputs.filename, e))?;
      let mut reader = BufReader::with_capacity(CAP, file);
  
      loop {
          let length = {
              let buffer = reader.fill_buf()?;
              // do stuff with buffer here
              let packet  =  Packet::V1(V1::success(&buffer)); // Packet::V1(V1::done())

              port.send(PacketWrapper {
               payload: packet,
                port: name.to_owned(),
              });

              buffer.len()
          };
          if length == 0 {
              break;
          }
          reader.consume(length);
      }
      */
       
      //let v : Vec<u8> = [20,33,44,44,90,33].into();
      /* 
      for x in contents.chunks(1024*1024) {
        debug!("X: {:?}", x);
        let packet  =  Packet::V1(V1::success(&x)); // Packet::V1(V1::done())

        port.send(PacketWrapper {
         payload: packet,
          port: name.to_owned(),
        });
      }
      
      port.send(PacketWrapper {
         payload: Packet::V1(V1::done()),
          port: name.to_owned(),
        });
        */
      outputs.contents.done(contents)?;
      
      Ok(())
    }
}
