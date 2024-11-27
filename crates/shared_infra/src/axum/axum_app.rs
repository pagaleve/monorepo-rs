use axum::{
    body::Body,
    extract::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};

use tracing::Instrument;

use super::axum_middlewares::log_request_response;
use axum::http::StatusCode;
use shared_common::error::http_custom_error::HttpCustomError;

#[derive(Clone)]
pub struct AxumApp {}

impl Default for AxumApp {
    fn default() -> Self {
        Self::new()
    }
}
impl AxumApp {
    pub fn new() -> Self {
        //The idea here in the near future is receive the subscribers as parameter, so we can change the subscriber to use opentelemetry (that is our main goal here)
        let subscriber = tracing_subscriber::fmt()
            .json()
            .with_target(true)
            .with_line_number(true)
            .flatten_event(true)
            .with_current_span(false)
            .without_time()
            .with_ansi(false)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();

        Self {}
    }

    pub async fn run(&self, router: Router) -> Router {
        Router::new()
            .nest("/api", router)
            .layer(middleware::from_fn(log_request_response))
            .layer(middleware::from_fn(tracing))
    }
}

async fn tracing(request: Request, next: Next) -> Response {
    let span = tracing::info_span!("request", method = %request.method(), uri = %request.uri());

    next.run(request).instrument(span).await
}
