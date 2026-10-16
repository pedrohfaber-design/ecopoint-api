use anyhow::Result;
use uuid::Uuid;

use crate::{
    app_state::AppState, dto::user_dashboard::UserDashboardDto,
    repositories::user_dashboard_repository::UserDashboardRepository,
};

pub struct UserDashboardService;

impl UserDashboardService {
    pub async fn stats(state: &AppState, user_id: Uuid) -> Result<UserDashboardDto> {
        UserDashboardRepository::stats(&state.pool, user_id).await
    }
}
