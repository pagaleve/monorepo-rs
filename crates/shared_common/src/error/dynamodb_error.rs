use crate::error::repository_error::RepositoryError;

impl From<aws_sdk_dynamodb::Error> for RepositoryError {
    fn from(error: aws_sdk_dynamodb::Error) -> Self {
        let message = format!("{:?}", error);

        let repository_error: RepositoryError = match error {
            aws_sdk_dynamodb::Error::TableNotFoundException(_) => {
                RepositoryError::TableNotFound(message)
            }
            aws_sdk_dynamodb::Error::ResourceNotFoundException(_) => {
                RepositoryError::ResourceNotFound(message)
            }
            aws_sdk_dynamodb::Error::ConditionalCheckFailedException(_) => {
                RepositoryError::ConditionalCheckFailed(message)
            }
            exception => RepositoryError::Unknown(Box::new(exception)),
        };

        repository_error
    }
}
