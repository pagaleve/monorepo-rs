use aws_sdk_dynamodb::types::AttributeValue;

pub struct DynamoDBUtil;
use aws_config::retry::RetryConfigBuilder;
impl DynamoDBUtil {
    pub fn get_item_request(
        client: &aws_sdk_dynamodb::Client,
        table_name: &str,
        p_key: (&str, &str),
        s_key: Option<(&str, &str)>,
    ) -> aws_sdk_dynamodb::operation::get_item::builders::GetItemFluentBuilder {
        let request = client
            .get_item()
            .table_name(table_name)
            .key(p_key.0, AttributeValue::S(p_key.1.to_string()));

        if let Some(s_key) = s_key {
            return request.key(s_key.0, AttributeValue::S(s_key.1.to_string()));
        }

        request
    }

    pub async fn get_dynamodb_client(retries: u32) -> aws_sdk_dynamodb::Client {
        let retry = RetryConfigBuilder::new().max_attempts(retries).build();
        let shared_config = aws_config::from_env().retry_config(retry).load().await;

        aws_sdk_dynamodb::Client::new(&shared_config)
    }

    pub async fn get_static_dynamodb_client(retries: u32) -> &'static aws_sdk_dynamodb::Client {
        let client = DynamoDBUtil::get_dynamodb_client(retries).await;

        Box::leak(Box::new(client))
    }
}
