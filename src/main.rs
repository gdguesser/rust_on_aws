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
    let config = aws_config::from_env().region(region_provider).load().await?;
    let client = Client::new(&config);

    let request = client.put_item()
        .table_name("questions")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("question", AttributeValue::S(String::from(event.question)))
        .item("answer", AttributeValue::S(String::from(event.answer)));

    request.send().await?;

    Ok(json!({"message": "Record written"}))
}