use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWasteTypeDto {
    pub name: String,
    pub description: Option<String>,
}
