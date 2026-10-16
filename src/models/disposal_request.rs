use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::request_status::RequestStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DisposalRequest {
    pub id: Uuid,

    pub user_id: Uuid,

    pub collection_point_id: Uuid,

    pub status: RequestStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
