/**********************************************
***** This file is generated, do not edit *****
***********************************************/
#![allow(unused_qualifications, unused_imports, missing_copy_implementations, unused_qualifications)]

use wasmflow_sdk::v1::ephemeral::BatchedJobExecutor;

#[cfg(all(target_arch = "wasm32"))]
type CallResult = wasmflow_sdk::v1::BoxedFuture<Result<Vec<u8>, wasmflow_sdk::v1::BoxedError>>;

#[cfg(all(target_arch = "wasm32"))]
#[allow(unsafe_code)]
#[no_mangle]
pub(crate) extern "C" fn wapc_init() {
  wasmflow_sdk::v1::wasm::runtime::register_dispatcher(Box::new(ComponentDispatcher::default()));
}



    pub mod manifest; // manifest
pub mod __batch__;

#[allow(unused)]
static  ALL_COMPONENTS: &[&str] = &[
"manifest",
];

#[derive(Default, Copy, Clone)]
#[allow(missing_debug_implementations)]
pub struct ComponentDispatcher {}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_lines)]
impl wasmflow_sdk::v1::ephemeral::WasmDispatcher for ComponentDispatcher {
  fn dispatch(
    &self,
    op: &'static str,
    payload: &'static [u8],
    ) -> CallResult {
    Box::pin(async move {
      let (mut stream, id) = match op {
          "manifest" => crate::components::generated::manifest::Component::default().execute(wasmflow_sdk::v1::payload::from_buffer(payload)?, ).await,
        _ => Err(wasmflow_sdk::v1::error::Error::ComponentNotFound(
                    op.to_owned(),
                    ALL_COMPONENTS.join(", "),
                ).into()),
      }?;
      while let Some(next) = wasmflow_sdk::v1::StreamExt::next(&mut stream).await {
        wasmflow_sdk::v1::wasm::port_send(&next.port, id, next.payload)?;
      }

      Ok(Vec::new())
    })
  }
}


#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_lines)]
impl wasmflow_sdk::v1::ephemeral::NativeDispatcher for ComponentDispatcher {
    fn dispatch(
        &self,
        invocation: wasmflow_sdk::v1::Invocation,
    ) -> wasmflow_sdk::v1::BoxedFuture<
        Result<
            wasmflow_sdk::v1::PacketStream,
            wasmflow_sdk::v1::BoxedError,
        >,
    > {
        Box::pin(async move {
      let (stream, _id) = match invocation.target.name() {
          "manifest" => crate::components::generated::manifest::Component::default().execute(wasmflow_sdk::v1::payload::from_invocation(invocation)?, ).await,
        "__batch__" => {
          crate::components::generated::__batch__::Component::default()
            .execute(wasmflow_sdk::v1::payload::from_invocation(invocation)?, )
            .await
        }
        op => Err(format!(
          "Component not found on this collection: {}",
          op
        ).into()),
      }?;
      Ok(stream)
    })
  }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_signature() -> wasmflow_sdk::v1::types::CollectionSignature {
  let mut components:std::collections::HashMap<String, wasmflow_sdk::v1::types::ComponentSignature> = std::collections::HashMap::new();

    components.insert("manifest".to_owned(), generated::manifest::signature());


  wasmflow_sdk::v1::types::CollectionSignature {
    name: Some("manifest".to_owned()),
    features: wasmflow_sdk::v1::types::CollectionFeatures {
      streaming: false,
      stateful: false,
      version: wasmflow_sdk::v1::types::CollectionVersion::V0
    },
    format: 1,
    version: "0.0.0".to_owned(),
    types: std::collections::HashMap::from([
    ]).into(),
    components: components.into(),
    wellknown: Vec::new(),
    config: wasmflow_sdk::v1::types::TypeMap::new()
  }
}

pub mod types {
  // no additional types
}
pub mod generated {

  
  
      // start component manifest
      pub mod manifest {
          // The user-facing implementation job impl.
          use crate::components::manifest as implementation;
  
        // The generated definition of inputs, outputs, config, et al.
        use super::manifest as definition;
        // The generated integration code between the definition and the implementation.
        use super::manifest as integration;
  
        
        pub use wasmflow_sdk::v1::Writable;
        pub use wasmflow_sdk::v1::ComponentOutput;
  
        pub use wasmflow_sdk::v1::packet::v1::Packet;
        pub use wasmflow_sdk::v1::console_log;
  
  
          #[derive(Default, Clone, Copy)]
          #[allow(missing_debug_implementations)]
          pub struct Component {}
          
          impl wasmflow_sdk::v1::Component for Component {
            type Inputs = definition::Inputs;
            type Outputs = definition::OutputPorts;
            type Config = integration::Config;
          }        
          
          impl wasmflow_sdk::v1::ephemeral::BatchedJobExecutor for Component {
            #[cfg(not(target_arch = "wasm32"))]
            type Payload = wasmflow_sdk::v1::packet::v1::PacketMap;
            #[cfg(target_arch = "wasm32")]
            type Payload = wasmflow_sdk::v1::wasm::EncodedMap;
            type Config = Config;
            type Return = (wasmflow_sdk::v1::PacketStream, u32);
            
          
            fn execute(
              &self,
              payload: wasmflow_sdk::v1::IncomingPayload<Self::Payload, Self::Config>,
              
            ) -> wasmflow_sdk::v1::BoxedFuture<Result<Self::Return, wasmflow_sdk::v1::BoxedError>> {
              Box::pin(async move {
                use wasmflow_sdk::v1::ephemeral::BatchedComponent;
                let id = payload.id();
                let (outputs, stream) = definition::get_outputs(id);
                let (payload, config) = payload.into_parts();
                let inputs = definition::convert_inputs(payload)?;
          
                Component::job(inputs, outputs,  config).await?;
                Ok((stream,id))
              })
            }
          }
  
        
        #[cfg(all(feature = "host", not(target_arch = "wasm32")))]
          pub fn signature() -> wasmflow_sdk::v1::types::ComponentSignature {
            wasmflow_sdk::v1::types::ComponentSignature {
              name: "manifest".to_owned(),
              inputs : inputs_list().into(),
              outputs : outputs_list().into(),
            }
          }      
        
        #[cfg(not(target_arch = "wasm32"))]
        pub fn convert_inputs(mut payload:wasmflow_sdk::v1::packet::v1::PacketMap) -> Result<definition::Inputs,Box<dyn std::error::Error + Send + Sync>> {
          Ok(definition::Inputs {
              source: payload.remove("source").ok_or_else(||wasmflow_sdk::v1::error::Error::MissingInput("source".to_owned()))?.deserialize()?,
          })
        }
        
        #[cfg(target_arch = "wasm32")]
        pub fn convert_inputs(payload:wasmflow_sdk::v1::wasm::EncodedMap) -> Result<definition::Inputs,Box<dyn std::error::Error + Send + Sync>> {
          Ok(definition::Inputs {
            source: wasmflow_sdk::v1::codec::messagepack::deserialize(payload.get("source")?)?,
          })
        }
        
        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct Inputs {
          #[serde(rename = "source")]
          pub source: 
        
          
          Vec<u8>
          
          
          
          ,
        }
        
        impl From<Inputs> for wasmflow_sdk::v1::packet::PacketMap {
          fn from(inputs: Inputs) -> wasmflow_sdk::v1::packet::PacketMap {
            let mut map = std::collections::HashMap::default();
              map.insert("source".to_owned(), wasmflow_sdk::v1::packet::v1::Packet::success(&inputs.source).into());
            wasmflow_sdk::v1::packet::PacketMap::new(map)
          }
        }
        
        #[must_use]
        #[cfg(all(feature="host", not(target_arch = "wasm32")))]
        pub fn inputs_list() -> std::collections::HashMap<String, wasmflow_sdk::v1::types::TypeSignature> {
          let mut map = std::collections::HashMap::new();
            map.insert("source".to_owned(),  
          
          
          
          
          
          
          
          
          
          
          
          
          wasmflow_sdk::v1::types::TypeSignature::Bytes
          
          
        
          
        
        
          
        
        
        
        
        );
          map
        }
        
        // A list of ports and their type signatures.
        #[must_use]
        #[cfg(feature="host")]
        pub fn outputs_list() -> std::collections::HashMap<String, wasmflow_sdk::v1::types::TypeSignature> {
          let mut map = std::collections::HashMap::new();
            map.insert("contenttype".to_owned(),  
          
          
          
          
          
          
          
          
          
          
          wasmflow_sdk::v1::types::TypeSignature::String
          
          
          
          
        
          
        
        
          
        
        
        
        
        );
            map.insert("proofs".to_owned(),  
          
          
          
          
          
          
          
          
          
          
          
          
          wasmflow_sdk::v1::types::TypeSignature::Bytes
          
          
        
          
        
        
          
        
        
        
        
        );
          map
        }
        
        // A list of output ports and their associated stream sender implementations.
        #[derive(Debug)]
        #[cfg(feature = "host")]
        pub struct OutputPorts {
            pub contenttype: ContenttypePortSender,
            pub proofs: ProofsPortSender,
        }
        
        impl OutputPorts {
          fn new(id: u32) -> Self {
            Self {
               contenttype: ContenttypePortSender::new(id),
               proofs: ProofsPortSender::new(id),
            }
          }
        }
        
        // Definition and implementation of each port's sender.
        #[derive(Debug)]
        #[cfg(feature = "host")]
        pub struct ContenttypePortSender {
            port: wasmflow_sdk::v1::PortChannel,
            id: u32,
        }
        
        #[cfg(feature = "host")]
        impl ContenttypePortSender {
            fn new(id:u32) -> Self {
                Self {
                    id,
                    port: wasmflow_sdk::v1::PortChannel::new("contenttype"),
                }
            }
        }
        
        #[cfg(all(feature = "host"))]
        impl wasmflow_sdk::v1::Writable for ContenttypePortSender {
            type PayloadType = 
        
          String
          
          
          
          
          ;
        
            fn get_port(&self) -> Result<&wasmflow_sdk::v1::PortChannel, wasmflow_sdk::v1::BoxedError> {
              if self.port.is_closed() {
                Err(Box::new(wasmflow_sdk::v1::error::Error::SendError("@key".to_owned())))
              } else {
                Ok(&self.port)
              }
            }
        
            fn get_port_name(&self) -> &str {
              &self.port.name
            }
        
            fn get_id(&self) -> u32 {
              self.id
            }
        }
        
        #[derive(Debug)]
        #[cfg(feature = "host")]
        pub struct ProofsPortSender {
            port: wasmflow_sdk::v1::PortChannel,
            id: u32,
        }
        
        #[cfg(feature = "host")]
        impl ProofsPortSender {
            fn new(id:u32) -> Self {
                Self {
                    id,
                    port: wasmflow_sdk::v1::PortChannel::new("proofs"),
                }
            }
        }
        
        #[cfg(all(feature = "host"))]
        impl wasmflow_sdk::v1::Writable for ProofsPortSender {
            type PayloadType = 
        
          
          Vec<u8>
          
          
          
          ;
        
            fn get_port(&self) -> Result<&wasmflow_sdk::v1::PortChannel, wasmflow_sdk::v1::BoxedError> {
              if self.port.is_closed() {
                Err(Box::new(wasmflow_sdk::v1::error::Error::SendError("@key".to_owned())))
              } else {
                Ok(&self.port)
              }
            }
        
            fn get_port_name(&self) -> &str {
              &self.port.name
            }
        
            fn get_id(&self) -> u32 {
              self.id
            }
        }
        
        
        #[cfg(all(feature = "host"))]
        pub fn get_outputs(id:u32) -> (OutputPorts, wasmflow_sdk::v1::PacketStream) {
            let mut outputs = OutputPorts::new(id);
            let mut ports = vec![
            &mut outputs.contenttype.port,
            &mut outputs.proofs.port,
            ];
            let stream = wasmflow_sdk::v1::PortChannel::merge_all(&mut ports);
            (outputs, stream)
        }
        
        #[allow(missing_debug_implementations)]
        pub struct Outputs {
          packets: ComponentOutput
        }
        
        impl Outputs {
            pub async fn contenttype(&mut self)-> Result<wasmflow_sdk::v1::PortOutput<
        
          String
          
          
          
          
          >, wasmflow_sdk::v1::error::Error> {
              let packets = self.packets.drain_port("contenttype").await?;
              Ok(wasmflow_sdk::v1::PortOutput::new("contenttype".to_owned(), packets))
            }
            pub async fn proofs(&mut self)-> Result<wasmflow_sdk::v1::PortOutput<
        
          
          Vec<u8>
          
          
          
          >, wasmflow_sdk::v1::error::Error> {
              let packets = self.packets.drain_port("proofs").await?;
              Ok(wasmflow_sdk::v1::PortOutput::new("proofs".to_owned(), packets))
            }
        }
        
        impl From<ComponentOutput> for Outputs {
          fn from(packets: ComponentOutput) -> Self {
            Self{packets}
          }
        }
        
        impl From<wasmflow_sdk::v1::PacketStream> for Outputs {
          fn from(stream: wasmflow_sdk::v1::PacketStream) -> Self {
            Self {
                packets: ComponentOutput::new(stream),
            }
          }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        impl From<wasmflow_sdk::v1::transport::TransportStream> for Outputs {
          fn from(stream: wasmflow_sdk::v1::transport::TransportStream) -> Self {
            Self {
                packets: ComponentOutput::new_from_ts(stream),
            }
          }
        }
        
        
        #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, Clone)]
        pub struct Config {
        }
      }
      // end component manifest
  
  
  pub mod __batch__ {
    use crate::components::__batch__ as implementation;
    use super::__batch__ as integration;
  
    use super::__batch__ as definition;
  
    
    pub use wasmflow_sdk::v1::Writable;
    pub use wasmflow_sdk::v1::ComponentOutput;
  
    pub use wasmflow_sdk::v1::packet::v1::Packet;
    pub use wasmflow_sdk::v1::console_log;
  
    
    
    impl wasmflow_sdk::v1::ephemeral::BatchedJobExecutor for Component {
      #[cfg(not(target_arch = "wasm32"))]
      type Payload = wasmflow_sdk::v1::packet::v1::PacketMap;
      #[cfg(target_arch = "wasm32")]
      type Payload = wasmflow_sdk::v1::wasm::EncodedMap;
      type Config = Config;
      type Return = (wasmflow_sdk::v1::PacketStream, u32);
      
    
      fn execute(
        &self,
        payload: wasmflow_sdk::v1::IncomingPayload<Self::Payload, Self::Config>,
        
      ) -> wasmflow_sdk::v1::BoxedFuture<Result<Self::Return, wasmflow_sdk::v1::BoxedError>> {
        Box::pin(async move {
          use wasmflow_sdk::v1::ephemeral::BatchedComponent;
          let id = payload.id();
          let (outputs, stream) = definition::get_outputs(id);
          let (payload, config) = payload.into_parts();
          let inputs = definition::convert_inputs(payload)?;
    
          Component::job(inputs, outputs,  config).await?;
          Ok((stream,id))
        })
      }
    }
  
    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub enum ComponentInputs {
        Manifest(super::manifest::Inputs),
    }
  
    #[cfg(all(feature = "guest"))]
    #[allow(missing_debug_implementations)]
    pub enum ComponentOutputs {
        Manifest(super::manifest::Outputs),
    }
  
    #[derive(Debug, serde::Deserialize)]
    pub enum Config {
        Manifest(super::manifest::Config),
    }
  
    #[derive(Default, Clone, Copy)]
    #[allow(missing_debug_implementations)]
    pub struct Component {}
    
    impl wasmflow_sdk::v1::Component for Component {
      type Inputs = definition::Inputs;
      type Outputs = definition::OutputPorts;
      type Config = integration::Config;
    }  
    // A list of ports and their type signatures.
    #[must_use]
    #[cfg(feature="host")]
    pub fn outputs_list() -> std::collections::HashMap<String, wasmflow_sdk::v1::types::TypeSignature> {
      let mut map = std::collections::HashMap::new();
        map.insert("result".to_owned(),  
      
      
      
      
      
      
      
      
      
      wasmflow_sdk::v1::types::TypeSignature::Bool
      
      
      
      
      
    
      
    
    
      
    
    
    
    
    );
      map
    }
    
    // A list of output ports and their associated stream sender implementations.
    #[derive(Debug)]
    #[cfg(feature = "host")]
    pub struct OutputPorts {
        pub result: ResultPortSender,
    }
    
    impl OutputPorts {
      fn new(id: u32) -> Self {
        Self {
           result: ResultPortSender::new(id),
        }
      }
    }
    
    // Definition and implementation of each port's sender.
    #[derive(Debug)]
    #[cfg(feature = "host")]
    pub struct ResultPortSender {
        port: wasmflow_sdk::v1::PortChannel,
        id: u32,
    }
    
    #[cfg(feature = "host")]
    impl ResultPortSender {
        fn new(id:u32) -> Self {
            Self {
                id,
                port: wasmflow_sdk::v1::PortChannel::new("result"),
            }
        }
    }
    
    #[cfg(all(feature = "host"))]
    impl wasmflow_sdk::v1::Writable for ResultPortSender {
        type PayloadType = 
    
      
      
      
      
      
      bool;
    
        fn get_port(&self) -> Result<&wasmflow_sdk::v1::PortChannel, wasmflow_sdk::v1::BoxedError> {
          if self.port.is_closed() {
            Err(Box::new(wasmflow_sdk::v1::error::Error::SendError("@key".to_owned())))
          } else {
            Ok(&self.port)
          }
        }
    
        fn get_port_name(&self) -> &str {
          &self.port.name
        }
    
        fn get_id(&self) -> u32 {
          self.id
        }
    }
    
    
    #[cfg(all(feature = "host"))]
    pub fn get_outputs(id:u32) -> (OutputPorts, wasmflow_sdk::v1::PacketStream) {
        let mut outputs = OutputPorts::new(id);
        let mut ports = vec![
        &mut outputs.result.port,
        ];
        let stream = wasmflow_sdk::v1::PortChannel::merge_all(&mut ports);
        (outputs, stream)
    }
    
    #[allow(missing_debug_implementations)]
    pub struct Outputs {
      packets: ComponentOutput
    }
    
    impl Outputs {
        pub async fn result(&mut self)-> Result<wasmflow_sdk::v1::PortOutput<
    
      
      
      
      
      
      bool>, wasmflow_sdk::v1::error::Error> {
          let packets = self.packets.drain_port("result").await?;
          Ok(wasmflow_sdk::v1::PortOutput::new("result".to_owned(), packets))
        }
    }
    
    impl From<ComponentOutput> for Outputs {
      fn from(packets: ComponentOutput) -> Self {
        Self{packets}
      }
    }
    
    impl From<wasmflow_sdk::v1::PacketStream> for Outputs {
      fn from(stream: wasmflow_sdk::v1::PacketStream) -> Self {
        Self {
            packets: ComponentOutput::new(stream),
        }
      }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    impl From<wasmflow_sdk::v1::transport::TransportStream> for Outputs {
      fn from(stream: wasmflow_sdk::v1::transport::TransportStream) -> Self {
        Self {
            packets: ComponentOutput::new_from_ts(stream),
        }
      }
    }
    
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn convert_inputs(mut payload:wasmflow_sdk::v1::packet::v1::PacketMap) -> Result<definition::Inputs,Box<dyn std::error::Error + Send + Sync>> {
      Ok(definition::Inputs {
          inputs: payload.remove("inputs").ok_or_else(||wasmflow_sdk::v1::error::Error::MissingInput("inputs".to_owned()))?.deserialize()?,
      })
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn convert_inputs(payload:wasmflow_sdk::v1::wasm::EncodedMap) -> Result<definition::Inputs,Box<dyn std::error::Error + Send + Sync>> {
      Ok(definition::Inputs {
        inputs: wasmflow_sdk::v1::codec::messagepack::deserialize(payload.get("inputs")?)?,
      })
    }
    
    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub struct Inputs {
      #[serde(rename = "inputs")]
      pub inputs: 
    
      
      
      
      
      
        Vec<
    
      
      
      
      
      
            ComponentInputs
      >
      ,
    }
    
    impl From<Inputs> for wasmflow_sdk::v1::packet::PacketMap {
      fn from(inputs: Inputs) -> wasmflow_sdk::v1::packet::PacketMap {
        let mut map = std::collections::HashMap::default();
          map.insert("inputs".to_owned(), wasmflow_sdk::v1::packet::v1::Packet::success(&inputs.inputs).into());
        wasmflow_sdk::v1::packet::PacketMap::new(map)
      }
    }
    
    #[must_use]
    #[cfg(all(feature="host", not(target_arch = "wasm32")))]
    pub fn inputs_list() -> std::collections::HashMap<String, wasmflow_sdk::v1::types::TypeSignature> {
      let mut map = std::collections::HashMap::new();
        map.insert("inputs".to_owned(),  
      
      
      
      
      
      
      
      
      
      
      
      
      
      
      
    
      
    
    
      
    
    
      wasmflow_sdk::v1::types::TypeSignature::List{
        element:Box::new(  
      
      
      
      
      
      
      
      
      
      
      
      
      
      
      
    
      
    
        wasmflow_sdk::v1::types::TypeSignature::Internal(
          wasmflow_sdk::v1::types::InternalType::ComponentInput
        )
    
      
    
    
    
    
    ),
      }
    
    
    );
      map
    }
  
  }
  
  

}
