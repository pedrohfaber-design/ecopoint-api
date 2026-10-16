use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WasteType {
    pub id: Uuid,

    pub name: String,

    pub description: Option<String>,
}
