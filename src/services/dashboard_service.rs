use anyhow::Result;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::dashboard_stats::DashboardStatsDto,
    models::user::User,
    repositories::{dashboard_repository::DashboardRepository, user_repository::UserRepository},
};

pub struct DashboardService;

impl DashboardService {
    pub async fn stats(state: &AppState) -> Result<DashboardStatsDto> {
        DashboardRepository::stats(&state.pool).await
    }

    pub async fn me(state: &AppState, id: Uuid) -> Result<User> {
        let user = UserRepository::find_by_id(&state.pool, id)
            .await?
            .ok_or(anyhow::anyhow!("Usuário não encontrado"))?;

        Ok(user)
    }
}
