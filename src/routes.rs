use crate::handlers::{handler_hello, handler_random};
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/random", get(handler_random))
}