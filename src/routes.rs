use crate::handlers::{handler_hello, handler_random, handler_search};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/random", get(handler_random))
        .route("/search", post(handler_search))
}
