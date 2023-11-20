use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AnimalQuery {
    pub species: String,
}
