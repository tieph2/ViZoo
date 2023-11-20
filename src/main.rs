#![allow(unused)]

mod handlers;
mod routes;
use anyhow::Result as AnyResult;
use std::net::SocketAddr;
use tracing::log::{info, trace};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};

use crate::routes::routes;
use reqwest::blocking::{Client, ClientBuilder};

#[tokio::main]
async fn main() -> AnyResult<()> {
    let app = routes();
    run().await.unwrap();
    Ok(())
}

async fn run() -> AnyResult<()> {
    let app = routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
