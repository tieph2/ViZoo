
use axum::{
    http::StatusCode,
    response::{IntoResponse,Html},
    routing::{get,post},
    Json, Router, Extension
};
use reqwest::blocking::{Client, ClientBuilder};




const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals";
const CAT_URL : &str = "https://catfact.ninja/fact";
pub async fn handler_hello() -> impl IntoResponse {
    println!("->>  {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World!!!</strong>")
}

pub async fn handler_random() -> impl IntoResponse {
    println!("->>  {:<12} - handler_random", "HANDLER");
    // let http_client : Client = Client::new();
    // let http_result = http_client.get( CAT_URL).send();
    //
    // if http_result.is_ok() {
    //     println!("{:#?}", http_result.ok().unwrap())
    // }

    Html("Hello <strong> World!!!</strong>")
}