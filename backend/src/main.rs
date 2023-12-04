mod handlers;
mod routes;
use crate::routes::routes;
use anyhow::Result as AnyResult;
use std::net::SocketAddr;
use tracing::log::info;

#[tokio::main]
async fn main() -> AnyResult<()> {
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
