use serde::{Deserialize, Serialize};

use crate::models::disposal_request::DisposalRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDashboardDto {
    pub pending: i64,
    pub approved: i64,
    pub completed: i64,
    pub cancelled: i64,

    pub last_requests: Vec<DisposalRequest>,
}
