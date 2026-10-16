use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWasteTypeDto {
    pub name: String,
    pub description: Option<String>,
}
