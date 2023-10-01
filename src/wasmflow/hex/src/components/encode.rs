
pub use crate::components::generated::encode::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

      let results = hex::encode(&inputs.input);
      outputs.output.done(results)?;

      Ok(())
    }
}
