use uuid::Uuid;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use serde_json::json;

use crate::{
    app_state::AppState, extractors::auth_user::AuthUser,
    services::dashboard_service::DashboardService,
};

pub async fn stats(State(state): State<AppState>) -> impl IntoResponse {
    match DashboardService::stats(&state).await {
        Ok(stats) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": stats
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

pub async fn me(State(state): State<AppState>, auth: AuthUser) -> impl IntoResponse {
    let id = match Uuid::parse_str(&auth.0.sub) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": "Token inválido"
                })),
            );
        }
    };

    match DashboardService::me(&state, id).await {
        Ok(user) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": user
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        ),
    }
}
