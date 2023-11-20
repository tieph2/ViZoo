use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use axum::extract::Query;
use reqwest::Error;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use vi_zoo::AnimalQuery;

const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals?name=";
const CAT_URL: &str = "https://catfact.ninja/fact";
const API_KEY : &str = "LX8eq5FkHB438N3K7ukLCw==DQTKEHV5lLP5rVtO";
pub async fn handler_hello() -> impl IntoResponse {
    println!("->>  {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World!!!</strong>")
}

pub async fn handler_random() -> Result<String, StatusCode> {
    let client = reqwest::Client::new();


    // Replace the URL below with the actual endpoint you want to call
    let response = client
        .get(CAT_URL)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(body)
}

pub async fn handler_animal() -> impl IntoResponse {
    println!("->>  {:<12} - handler_random", "HANDLER");
    Html("Hello <strong> World!!!</strong>")
}

pub async fn handler_search(Json(body): Json<AnimalQuery>) -> Result<String, StatusCode> {
    // Perform a search operation based on the provided animal name
    // For demonstration purposes, let's assume a hardcoded list of animals
    println!("->>  {:<12} - handler_search", "HANDLER");
    let available_animals = vec!["dog", "cat", "rabbit", "lion"];


    let client = reqwest::Client::new();

    //Construct the header for the request
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(API_KEY).unwrap());

    //Construct the final url by adding the name of the species to API url:
    let species = &*body.species.to_lowercase();
    let url = [ANIMAL_URL, species].concat();
    println!("{}", url);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Replace the URL below with the actual endpoint you want to call
    // let response = client
    //     .get(CAT_URL)
    //     .send()
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //
    let body = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(body)


    // if available_animals.contains(&&*body.species.to_lowercase()) {
    //     println!("{}", body.species);
    //     let species = body.species;
    //     let concatenated_str = [ANIMAL_URL, species.as_str()].concat();
    //
    //     Ok(format!("Animal '{}' found!", concatenated_str))
    // } else {
    //     Err(StatusCode::NOT_FOUND)
    // }
}
