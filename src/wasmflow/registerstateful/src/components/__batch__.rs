pub use crate::components::generated::__batch__::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::stateful::BatchedComponent for Component {
    type Context = crate::Context;

    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        context: Self::Context,
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
