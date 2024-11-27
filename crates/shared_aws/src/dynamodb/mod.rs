pub mod util;

use aws_config::retry::RetryConfigBuilder;

pub async fn get_dynamodb_client(retries: u32) -> aws_sdk_dynamodb::Client {
    let retry = RetryConfigBuilder::new().max_attempts(retries).build();
    let shared_config = aws_config::from_env().retry_config(retry).load().await;

    aws_sdk_dynamodb::Client::new(&shared_config)
}
