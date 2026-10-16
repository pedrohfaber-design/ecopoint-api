use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub request_id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}