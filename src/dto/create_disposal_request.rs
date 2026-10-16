use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisposalRequestDto {
    pub user_id: Uuid,

    pub collection_point_id: Uuid,

    pub items: Vec<CreateDisposalItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisposalItemDto {
    pub waste_type_id: Uuid,

    pub quantity: i32,
}
