#![allow(unused)]

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

use reqwest::blocking::{Client, ClientBuilder};




const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals";
const CAT_URL : &str = "https://catfact.ninja/fact";

#[tokio::main]
async fn main() {

    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong> World!!</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}


// fn main() {
//
//     let http_client : Client = Client::new();
//     let http_result = http_client.get( CAT_URL).send();
//
//     if http_result.is_ok() {
//         println!("{:#?}", http_result.ok().unwrap())
//     }
// }
