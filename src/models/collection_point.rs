use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectionPoint {
    pub id: Uuid,

    pub name: String,

    pub description: String,

    pub address: String,

    pub city: String,

    pub state: String,

    pub latitude: f64,

    pub longitude: f64,

    pub phone: Option<String>,

    pub email: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
