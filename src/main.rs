use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func  = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CustomEvent {
    question: String,
    answer: String
}

async fn handler(event: CustomEvent, _: Context) -> Result<Value, LambdaError> {
    Uuid::new_v4().to_string();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
}