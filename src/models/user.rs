use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::user_role::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,

    pub name: String,

    pub email: String,

    pub password_hash: String,

    pub phone: Option<String>,

    pub city: String,

    pub state: String,

    pub role: UserRole,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}
