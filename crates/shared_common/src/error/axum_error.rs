use crate::error::http_custom_error::HttpCustomError;
use axum::http::StatusCode;
use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
impl IntoResponse for HttpCustomError {
    fn into_response(self) -> Response<axum::body::Body> {
        let body = serde_json::to_string(&HttpCustomError {
            status: self.status,
            message: self.message,
            name: self.name,
            data: self.data,
            source: self.source,
        })
        .unwrap();

        let body = Body::from(body);

        Response::builder()
            .status(StatusCode::from_u16(self.status).unwrap())
            .body(body)
            .unwrap()
    }
}
