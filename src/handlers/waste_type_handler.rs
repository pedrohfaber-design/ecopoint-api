use crate::{extractors::auth_user::AuthUser, utils::auth::require_admin};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::{create_waste_type::CreateWasteTypeDto, update_waste_type::UpdateWasteTypeDto},
    services::waste_type_service::WasteTypeService,
};

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(dto): Json<CreateWasteTypeDto>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }

    match WasteTypeService::create(&state, &dto).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Tipo de resíduo criado com sucesso."
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match WasteTypeService::find_all(&state).await {
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

pub async fn find_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match WasteTypeService::find_by_id(&state, id).await {
        Ok(Some(data)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": data
            })),
        ),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Tipo de resíduo não encontrado."
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
pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateWasteTypeDto>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }

    match WasteTypeService::update(&state, id, &dto).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Tipo de resíduo atualizado."
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}
pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }

    match WasteTypeService::delete(&state, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Tipo de resíduo removido."
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}
