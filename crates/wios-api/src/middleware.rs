//! API middleware for authentication, rate limiting, etc.

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

/// Logging middleware that records request details.
pub async fn request_logger(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("{} {}", method, uri);
    next.run(request).await
}
