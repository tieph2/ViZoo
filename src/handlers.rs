use axum::{
    http::StatusCode, extract::Query,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use reqwest::{
    Client,
    ClientBuilder,
    Error,
    header::{HeaderMap, HeaderValue},
};

use csv;
use std::error;
use std::fs::File;
use std::path::Path;
use csv::Reader;
use rand::Rng;
use vi_zoo::AnimalQuery;




const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals?name=";
const CAT_URL: &str = "https://catfact.ninja/fact";
const API_KEY : &str = "LX8eq5FkHB438N3K7ukLCw==DQTKEHV5lLP5rVtO";

/// Simple function to test server
pub async fn handler_hello() -> impl IntoResponse {
    println!("->>  {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World!!!</strong>")
}

fn read_random_record(file_path: &str) -> Result<csv::StringRecord, Box<dyn std::error::Error>> {
    println!("Is this even being called at all?");


    // Open the CSV file
    let file = File::open(file_path)?;

    // Create a CSV reader from the file
    let mut reader = Reader::from_reader(file);

    // Collect records into a vector
    let records: Vec<csv::StringRecord> = reader.records().collect::<Result<_, _>>()?;

    // Generate a random index within the range of records
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..records.len());


    for result in reader.records() {
        println!("{:?}", result);
        // Handle each CSV record
        match result {
            Ok(record) => {
                // Process the CSV record (access fields by index or by name if using headers)
                println!("{:?}", record);
            }
            Err(err) => {
                // Handle CSV parsing errors
                eprintln!("Error parsing CSV record: {}", err);
            }
        }
    }

    // Return the randomly selected record
    Ok(records[random_index].clone()) // Cloning the record to return it
}

///This function calls the Animal API to get a random animal
pub async fn handler_random() -> Result<String, StatusCode> {

    if let Ok(record) = read_random_record("./src/data/animals.csv") {
        println!("{:?}", record);
    } else {
        eprintln!("Error reading CSV file");
    }

    // let random_animal = read_random_record("./data/animals.csv").unwrap();
    // println!("{:?}", random_animal);

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

/// This function takes a name of an animal in the Json request and
/// call the Anial API to get information about that animal
pub async fn handler_search(Json(body): Json<AnimalQuery>) -> Result<String, StatusCode> {
    // Perform a search operation based on the provided animal name
    // For demonstration purposes, let's assume a hardcoded list of animals
    println!("->>  {:<12} - handler_search", "HANDLER");
    let available_animals = vec!["dog", "cat", "rabbit", "lion"];


    let client = reqwest::Client::new();

    //Construct the header for the request with the API key
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(API_KEY).unwrap());

    //Construct the final url by adding the name of the species to API url:
    let species = &*body.species.to_lowercase();
    let url = [ANIMAL_URL, species].concat();
    println!("{}", url);

    //Call the api
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //Parse the body
    let body = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("{}", body);
    if body == "[]" {
        println!("Empty response body");
        return Err(StatusCode::NOT_FOUND)
    }

    Ok(body)
}
