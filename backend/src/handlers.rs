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

use csv::Reader;
use rand::Rng;
use select::document::Document;
use select::predicate::Name;
use std::error;
use std::fs::File;
use std::path::Path;
use vi_zoo::{AnimalQuery, AnimalRecord};

const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals?name=";
const CAT_URL: &str = "https://catfact.ninja/fact";
const API_KEY : &str = "LX8eq5FkHB438N3K7ukLCw==DQTKEHV5lLP5rVtO";

// fn fetch_image_url(search_query: &str) -> Result<String, reqwest::Error> {
//     let client = Client::new();
//     let search_url = format!("https://www.google.com/search?tbm=isch&q={}", search_query);
//
//     let response = client.get(&search_url).send();
//     let body = response.text();
//
//     let document = Document::from(body.as_str());
//     let img_element = document.find(Name("img")).next();
//
//     if let Some(img) = img_element {
//         if let Some(img_url) = img.attr("src") {
//             return Ok(img_url.to_string());
//         }
//     }
//
//     Err(Error)
// }


/// Simple function to test server
pub async fn handler_hello() -> impl IntoResponse {
    println!("->>  {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World!!!</strong>")
}

///This function calls the Random Cat API (for testing purpose)
pub async fn handler_rand_cat() -> Result<String, StatusCode> {

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

async fn read_random_record(file_path: &str) -> Result<csv::StringRecord, Box<dyn std::error::Error>> {
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



///This function search for an animal given a name with the Animal API
async fn search_animal(name: &str) -> Result<String, StatusCode> {

    let client = reqwest::Client::new();

    //Construct the header for the request with the API key
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(API_KEY).unwrap());

    let url = [ANIMAL_URL, name].concat();
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
    }

    Ok(body)
}

/// This function takes a name of an animal in the Json request and
/// call the Animal API to get information about that animal
pub async fn handler_search(Json(body): Json<AnimalQuery>) -> Result<String, StatusCode> {

    //Construct the final url by adding the name of the species to API url:
    let species = &*body.species.to_lowercase();

    let body = search_animal(species).await;

    match body {
        Ok(body) => return Ok(body),
        Err(e) => Err(e)
    }
}

///This function calls the Animal API to get a random animal
pub async fn handler_random() -> Result<String, StatusCode> {
    let animal = read_random_record("./src/data/animals.csv");
    let mut name = String::new();
    match animal.await {
        Ok(record) => {
            println!("{:?}", record);
            let my_struct: AnimalRecord = record.deserialize(None).unwrap();
            println!("{:?}", my_struct);
            name = record.get(0).unwrap().to_string();
        }
        _ => {
            eprintln!("Error reading CSV file");
        }
    }


    let res = search_animal(name.as_str()).await;

    match res {
        Ok(body) => return Ok(body),
        Err(e) => Err(e)
    }

}