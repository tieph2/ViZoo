use axum::{
    http::StatusCode, extract::Query,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use reqwest::{
    Client,
    ClientBuilder,
    Error as RError,
    header::{HeaderMap, HeaderValue},
};

use csv::Reader;
use rand::Rng;
use select::document::Document;
use select::predicate::Name;
use std::error;
use std::fs::File;
use std::path::Path;
use vi_zoo::{AnimalQuery, AnimalRecord, Animal, AnimalResponse};
use serde::{Deserialize, Serialize};
use image_search::{Arguments, Color, urls, search, download};

const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals?name=";
const CAT_URL: &str = "https://catfact.ninja/fact";
const API_KEY : &str = "LX8eq5FkHB438N3K7ukLCw==DQTKEHV5lLP5rVtO";

async fn get_url(animal: String) -> Result<String, image_search::Error> {
    let args = Arguments::new(animal.as_str(), 10);
    let _image_urls = urls(args.clone()).await?;
    println!("{:?}", _image_urls[0]);

    Ok(_image_urls[0].clone())
}

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
    println!("{}", name);
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


    let mut result : Result<String,StatusCode> =
        //Continuously loop through random animal in the CSV database until a match in the
        // Animal API is found
        loop {
            let animal = read_random_record("./src/data/animals.csv");
            let mut name = String::new();
            match animal.await {
                Ok(record) => {
                    let my_struct: AnimalRecord = record.deserialize(None).unwrap();
                    name = record.get(0).unwrap().to_string();
                }
                _ => {
                    eprintln!("Error reading CSV file");
                }
            }

            let res = search_animal(name.as_str()).await;
            if res.as_ref().unwrap() != "[]" {
                break res;
            }

        };

    match result {
        Ok(body) => {
            let animals : Vec<Animal> = serde_json::from_str(body.as_str()).expect("Failed to deserialize JSON");
            let mut response : Vec<AnimalResponse> = Vec::new();
            for animal in animals {
                let res = parse_animal(&animal).await;
                if let Ok(animal) = res {
                    response.push(animal);
                }
            }
            let json_body = serde_json::to_string(&response).expect("Failed to serialize");
            return Ok(json_body)
        },
        Err(e) => Err(e)
    }

}

async fn parse_animal(animal : &Animal) -> Result<AnimalResponse, Box<dyn std::error::Error>> {

    println!("Getting url");
    let url = get_url(animal.name.clone().unwrap()).await?;

    let response = AnimalResponse  {
        name: animal.name.clone(),
        scientific_name: animal.taxonomy.scientific_name.clone(),
        locations: animal.locations.clone(),
        prey: animal.characteristics.prey.clone(),
        diet: animal.characteristics.diet.clone(),
        r#type: animal.characteristics.r#type.clone(),
        color: animal.characteristics.color.clone(),
        lifespan: animal.characteristics.lifespan.clone(),
        weight: animal.characteristics.weight.clone(),
        img_url:  url
    };
    Ok(response)
}