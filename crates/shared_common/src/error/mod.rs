#[cfg(feature = "axum_error")]
pub mod axum_error;
#[cfg(feature = "dynamodb_error")]
pub mod dynamodb_error;
pub mod http_custom_error;
pub mod repository_error;
