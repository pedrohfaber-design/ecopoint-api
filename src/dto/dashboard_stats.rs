use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStatsDto {
    pub total_users: i64,
    pub total_collection_points: i64,
    pub total_waste_types: i64,

    pub pending_requests: i64,
    pub approved_requests: i64,
    pub completed_requests: i64,
    pub cancelled_requests: i64,
}
