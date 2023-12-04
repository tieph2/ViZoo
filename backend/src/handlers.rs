use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json
};

use reqwest::{
    header::{HeaderMap, HeaderValue}
};

use csv::Reader;
use image_search::{urls, Arguments};
use rand::Rng;
use std::fs::File;
use vi_zoo::{Animal, AnimalQuery, AnimalResponse};

const ANIMAL_URL: &str = "https://api.api-ninjas.com/v1/animals?name=";
const API_KEY: &str = "LX8eq5FkHB438N3K7ukLCw==DQTKEHV5lLP5rVtO";

///This function get the url of the first result returned by google image search.
///The argument is a String representing the search query.
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


///This function opens a CSV file, reads it then return a random record representing an animal
async fn read_random_record(
    file_path: &str,
) -> Result<csv::StringRecord, Box<dyn std::error::Error>> {
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
/// calls the Animal API to get information about that animal
pub async fn handler_search(Json(body): Json<AnimalQuery>) -> Result<String, StatusCode> {
    //Construct the final url by adding the name of the species to API url:
    let species = &*body.species.to_lowercase();

    let result = search_animal(species).await;

    match result {
        Ok(body) => Ok(parse_result(body).await?),
        Err(e) => Err(e),
    }
}

///This function calls the Animal API to get a random animal
pub async fn handler_random() -> Result<String, StatusCode> {
    let result : Result<String,StatusCode> =
        //Continuously loop through random animal in the CSV database until a match in the
        // Animal API is found
        loop {
            let animal = read_random_record("./src/data/animals.csv");
            let mut name = String::new();
            match animal.await {
                Ok(record) => {
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
        Ok(body) => Ok(parse_result(body).await?),
        Err(e) => Err(e),
    }
}

///This function parses the result from the API call into a JSON response.
async fn parse_result(body: String) -> Result<String, StatusCode> {
    let animals: Vec<Animal> =
        serde_json::from_str(body.as_str()).expect("Failed to deserialize JSON");
    let mut response: Vec<AnimalResponse> = Vec::new();
    for animal in animals {
        let res = parse_animal(&animal).await;
        if let Ok(animal) = res {
            response.push(animal);
        }
    }
    let json_body = serde_json::to_string(&response).expect("Failed to serialize");
    Ok(json_body)
}

///This function parses the animal from the API call result to get the necessary info
async fn parse_animal(animal: &Animal) -> Result<AnimalResponse, Box<dyn std::error::Error>> {
    println!("Getting url");
    let url = get_url(animal.name.clone().unwrap()).await?;

    let response = AnimalResponse {
        name: animal.name.clone(),
        scientific_name: animal.taxonomy.scientific_name.clone(),
        locations: animal.locations.clone(),
        prey: animal.characteristics.prey.clone(),
        diet: animal.characteristics.diet.clone(),
        r#type: animal.characteristics.r#type.clone(),
        color: animal.characteristics.color.clone(),
        lifespan: animal.characteristics.lifespan.clone(),
        weight: animal.characteristics.weight.clone(),
        img_url: url,
    };
    Ok(response)
}
