use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DisposalItem {
    pub id: Uuid,

    pub request_id: Uuid,

    pub waste_type_id: Uuid,

    pub quantity: i32,
}
