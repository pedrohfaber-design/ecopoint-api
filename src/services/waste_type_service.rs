use anyhow::Result;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::{create_waste_type::CreateWasteTypeDto, update_waste_type::UpdateWasteTypeDto},
    models::waste_type::WasteType,
    repositories::waste_type_repository::WasteTypeRepository,
};

pub struct WasteTypeService;

impl WasteTypeService {
    pub async fn create(state: &AppState, dto: &CreateWasteTypeDto) -> Result<()> {
        WasteTypeRepository::create(&state.pool, dto).await
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<WasteType>> {
        WasteTypeRepository::find_all(&state.pool).await
    }

    pub async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<WasteType>> {
        WasteTypeRepository::find_by_id(&state.pool, id).await
    }

    pub async fn update(state: &AppState, id: Uuid, dto: &UpdateWasteTypeDto) -> Result<()> {
        WasteTypeRepository::update(&state.pool, id, dto).await
    }

    pub async fn delete(state: &AppState, id: Uuid) -> Result<()> {
        WasteTypeRepository::delete(&state.pool, id).await
    }
}
