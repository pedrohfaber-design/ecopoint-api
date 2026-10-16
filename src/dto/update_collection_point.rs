use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCollectionPointDto {
    pub name: Option<String>,

    pub description: Option<String>,

    pub address: Option<String>,

    pub city: Option<String>,

    pub state: Option<String>,

    pub latitude: Option<f64>,

    pub longitude: Option<f64>,

    pub phone: Option<String>,

    pub email: Option<String>,
}
