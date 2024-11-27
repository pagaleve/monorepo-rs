use axum::body::Body;
use axum::body::Bytes;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)^[\s--\r\n\\]*").unwrap();
}
pub async fn log_request_response(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut do_log = true;

    let path = &req.uri().path().to_string();

    // Don't log these extensions
    let extension_skip = vec![".js", ".html", ".css", ".png", ".jpeg"];
    for ext in extension_skip {
        if path.ends_with(ext) {
            do_log = false;
            break;
        }
    }

    // Want to skip logging these paths
    let skip_paths = vec!["/example/path"];
    for skip_path in skip_paths {
        if path.ends_with(skip_path) {
            do_log = false;
            break;
        }
    }

    let (req_parts, req_body) = req.into_parts();

    let bytes = buffer_and_print("request", req_body, do_log, None).await?;
    let req = Request::from_parts(req_parts, axum::body::Body::from(bytes));

    let res = next.run(req).await;

    let (mut res_parts, res_body) = res.into_parts();

    // Print response
    let bytes = buffer_and_print("response", res_body, do_log, Some(res_parts.status)).await?;

    // When your encoding is chunked there can be problems without removing the header
    res_parts.headers.remove("transfer-encoding");

    let res = Response::from_parts(res_parts, Body::from(bytes));

    Ok(res)
}

// Consumes body and prints
async fn buffer_and_print(
    direction: &str,
    body: axum::body::Body,
    log: bool,
    status_code: Option<StatusCode>,
) -> Result<Bytes, (StatusCode, String)> {
    let bytes = match axum::body::to_bytes(body, 50000).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    // Print request and response
    // The goal here is to print as JSON not string, but I could not print the body as json yet, it is always logging as ""{\"key\": \"value\"}\"
    if let Ok(body) = std::str::from_utf8(&bytes) {
        if log && !body.is_empty() {
            if body.len() > 50000 {
                tracing::warn!(from = direction, "Payload is too big");
            } else if (status_code.is_some()) {
                tracing::info!(
                    from = direction,
                    status_code = status_code.unwrap().as_u16(),
                    body = RE.replace_all(body, "").to_string().trim()
                );
            } else {
                tracing::info!(
                    from = direction,
                    body = RE.replace_all(body, "").to_string().trim()
                );
            }
        }
    }

    Ok(bytes)
}
