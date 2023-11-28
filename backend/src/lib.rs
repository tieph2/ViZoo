use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AnimalQuery {
    pub species: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalRecord {
    name: String,
    height: String,
    weight: String,
    color: String,
    lifespan: String,
    diet: String,
    habitat: String,
    predators: String,
    avg_speed: String,
    countries: String,
    status: String,
    family: String,
    gestation_period: String,
    top_speed: String,
    social_structure: String,
    offsprings: String
}