
pub use crate::components::generated::get::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::stateful::BatchedComponent for Component {
    type Context= crate::Context;

    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        context: Self::Context,
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

      if let Some(result) = context.map.get(&inputs.key){
        outputs.value.done((*result).to_owned())?;
      }

      Ok(())
    }
}
