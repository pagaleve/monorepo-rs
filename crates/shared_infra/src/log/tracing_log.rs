use serde::Serialize;
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct LogData<T: Serialize> {
    message: String,
    level: String,
    timestamp: String,
    data: T,
}

impl<T> LogData<T>
where
    T: Serialize,
{
    pub fn info(message: &str, data: Option<&T>) {
        info!(
            "{}",
            serde_json::to_string(&LogData {
                message: message.to_string(),
                level: "info".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                data
            })
            .unwrap()
        )
    }

    pub fn error(message: &str, data: Option<&T>, _panic: bool) {
        error!(
            "{}",
            serde_json::to_string(&LogData {
                message: message.to_string(),
                level: "error".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                data
            })
            .unwrap()
        )
    }
}
