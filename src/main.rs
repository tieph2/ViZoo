#![allow(unused)]

mod handlers;
mod routes;

use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse,Html},
    routing::{get,post},
    Json, Router
};

use reqwest::blocking::{Client, ClientBuilder};
use crate::routes::routes;


#[tokio::main]
async fn main() {

    let app = routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


