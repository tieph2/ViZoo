use crate::handlers::{handler_hello, handler_random, handler_search};
use axum::{
    routing::{get, post},
    Router,
};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};


pub fn routes() -> Router {

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method:: POST])
        .allow_origin(Any)
        .allow_headers(Any);


    Router::new()
        .route("/hello", get(handler_hello))
        .route("/random", get(handler_random))
        .route("/search", post(handler_search))
        .layer(cors)
}
