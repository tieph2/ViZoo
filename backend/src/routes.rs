use crate::handlers::{handler_hello, handler_random, handler_search};
use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

///Routes for the webserver.
pub fn routes() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/hello", get(handler_hello))
        .route("/random", get(handler_random))
        .route("/search", post(handler_search))
        .layer(cors)
}
