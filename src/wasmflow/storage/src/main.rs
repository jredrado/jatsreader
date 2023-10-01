use async_trait::async_trait;
use wasmflow_rpc::error::RpcError;
use wasmflow_rpc::{RpcHandler, RpcResult};
use wasmflow_sdk::v1::stateful::NativeDispatcher;
use wasmflow_sdk::v1::transport::TransportStream;
use wasmflow_sdk::v1::types::HostedType;
use wasmflow_sdk::v1::Invocation;

use self::components::ComponentDispatcher;
pub mod components;

#[macro_use]
extern crate tracing;

use chashmap::CHashMap;

#[derive(Clone, Debug)]
pub struct Context {
    map: Arc<CHashMap<Vec<u8>, Vec<u8>>>
}

#[derive(Clone)]
pub struct Collection {
  context: Context,
}

impl Collection {
  pub fn default() -> Self {
    Self { context: Context {
        map : Arc::new(CHashMap::new())
    } }
  }
}

#[async_trait]
impl RpcHandler for Collection {
  async fn invoke(&self, invocation: Invocation) -> RpcResult<TransportStream> {
    let target = invocation.target_url();
    trace!("test collection invoke: {}", target);
    let context = self.context.clone();
    let dispatcher = ComponentDispatcher::default();
    let result = dispatcher
      .dispatch(invocation, context)
      .await
      .map_err(|e| RpcError::CollectionError(e.to_string()));
    trace!("test collection result: {}", target);
    let stream = result?;

    Ok(TransportStream::from_packetstream(stream))
  }

  fn get_list(&self) -> RpcResult<Vec<HostedType>> {
    trace!("test collection get list");
    let signature = components::get_signature();
    Ok(vec![HostedType::Collection(signature)])
  }
}

use std::sync::Arc;

use clap::Parser;
use wasmflow_collection_cli::options::DefaultCliOptions;

#[derive(Debug, Clone, Parser)]
pub struct Options {
  /// IP address to bind to.
  //#[clap(short = 'u', long = "redis-url", env = wasmflow_keyvalue_redis::REDIS_URL_ENV, action)]
  //pub url: String,

  #[clap(flatten)]
  pub options: DefaultCliOptions,
}

#[tokio::main]
async fn main() -> Result<(), wasmflow_collection_cli::Error> {
  let opts = Options::parse();
  //let url = opts.url;
  let _guard = wasmflow_collection_cli::init_logging(&opts.options.logging.name("storage"));
  let collection = Collection::default();
  //collection.connect("default".to_owned(), url.clone()).await?;
  trace!("storage collection connected");

  wasmflow_collection_cli::init_cli(Arc::new(collection), Some(opts.options.into())).await?;
  Ok(())
}