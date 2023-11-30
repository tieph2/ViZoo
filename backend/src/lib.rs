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
#[derive(Debug, Deserialize, Serialize)]
pub struct Taxonomy {
    pub kingdom: Option<String>,
    pub phylum: Option<String>,
    pub class: Option<String>,
    pub order: Option<String>,
    pub family: Option<String>,
    pub genus: Option<String>,
    pub scientific_name: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Characteristics {
    pub prey: Option<String>,
    pub group_behavior: Option<String>,
    pub estimated_population_size: Option<String>,
    pub biggest_threat: Option<String>,
    pub most_distinctive_feature: Option<String>,
    pub other_names: Option<String>,
    pub water_type: Option<String>,
    pub habitat: Option<String>,
    pub predators: Option<String>,
    pub diet: Option<String>,
    pub r#type: Option<String>,
    pub common_name: Option<String>,
    pub number_of_species: Option<String>,
    pub color: Option<String>,
    pub skin_type: Option<String>,
    pub lifespan: Option<String>,
    pub weight: Option<String>,
    pub length: Option<String>,
    pub height: Option<String>,
    pub average_litter_size: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Animal {
    pub name: Option<String>,
    pub taxonomy: Taxonomy,
    pub locations: Vec<String>,
    pub characteristics: Characteristics,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AnimalResponse {
    pub name: String,
    pub scientific_name: String,
    pub locations: Vec<String>,
    pub prey: String,
    pub diet: String,
    pub r#type: String,
    pub color: String,
    pub lifespan: String,
    pub weight: String,
    pub img_url: String
}