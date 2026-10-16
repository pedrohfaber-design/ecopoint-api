use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;
use uuid::Uuid;

use crate::{app_state::AppState, services::user_dashboard_service::UserDashboardService};

pub async fn stats(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    match UserDashboardService::stats(&state, user_id).await {
        Ok(data) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": data
            })),
        ),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        ),
    }
}
