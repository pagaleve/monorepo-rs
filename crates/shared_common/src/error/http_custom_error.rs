use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct HttpCustomError {
    pub status: u16,
    pub message: String,
    #[serde(rename = "type")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub data: Option<HashMap<&'static str, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl Display for HttpCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {:?}", self.message, self.data)
    }
}

impl std::error::Error for HttpCustomError {}

impl HttpCustomError {
    pub fn new(
        status: u16,
        message: String,
        name: String,
        data: Option<HashMap<&'static str, String>>,
    ) -> Self {
        HttpCustomError {
            status,
            message,
            name,
            data,
            source: None,
        }
    }
}
