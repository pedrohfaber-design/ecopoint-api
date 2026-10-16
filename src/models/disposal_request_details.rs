use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request_status::RequestStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisposalRequestDetails {
    pub id: Uuid,
    pub user_id: Uuid,

    pub collection_point_id: Uuid,
    pub collection_point_name: String,
    pub collection_point_city: String,
    pub collection_point_state: String,

    pub status: RequestStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub items: Vec<DisposalRequestItemDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisposalRequestItemDetails {
    pub id: Uuid,
    pub waste_type_id: Uuid,
    pub waste_type_name: String,
    pub quantity: i32,
}