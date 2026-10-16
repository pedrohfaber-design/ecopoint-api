use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionPointDto {
    pub name: String,

    pub description: String,

    pub address: String,

    pub city: String,

    pub state: String,

    pub latitude: f64,

    pub longitude: f64,

    pub phone: Option<String>,

    pub email: Option<String>,
}
