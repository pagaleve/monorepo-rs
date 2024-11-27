use thiserror::Error;
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("{0}")]
    TableNotFound(String),

    #[error("{0}")]
    ResourceNotFound(String),

    #[error("{0}")]
    ConditionalCheckFailed(String),

    #[error("{0}")]
    Unknown(#[from] Box<dyn std::error::Error>),
}
