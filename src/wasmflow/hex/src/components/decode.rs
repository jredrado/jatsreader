
pub use crate::components::generated::decode::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

      let result = hex::decode(&inputs.input);

      match result {
        Ok(data) => {
                outputs.output.done(data)?;
                Ok(())
        },
        Err(e) => {
            Err(Box::new(e))
        }
      }

    }
}
