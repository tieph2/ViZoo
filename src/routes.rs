use crate::handlers::{handler_animal, handler_hello, handler_random, handler_search};
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/animal", get(handler_animal))
        .route("/random", get(handler_random))
        .route("/search", post(handler_search))
}
