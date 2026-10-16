use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::extractors::auth_user::AuthUser;
use serde_json::json;
use uuid::Uuid;
use crate::{
    app_state::AppState, dto::create_disposal_request::CreateDisposalRequestDto,
    models::request_status::RequestStatus,
    services::disposal_request_service::DisposalRequestService,
};

/// POST /api/disposal-requests
pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateDisposalRequestDto>,
) -> impl IntoResponse {
    match DisposalRequestService::create(&state.pool, dto).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Solicitação criada com sucesso."
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

/// GET /api/disposal-requests
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match DisposalRequestService::list(&state.pool).await {
        Ok(requests) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": requests
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

/// GET /api/disposal-requests/:id/details
pub async fn find_details_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match DisposalRequestService::find_details_by_id(&state.pool, id).await {
        Ok(Some(request)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": request
            })),
        )
            .into_response(),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Solicitação não encontrada."
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}

/// GET /api/disposal-requests/my
pub async fn my_requests(
    State(state): State<AppState>,
    auth: AuthUser,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth.0.sub) {
        Ok(id) => id,

        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "ID do usuário inválido no token."
                })),
            )
                .into_response();
        }
    };

    match DisposalRequestService::list_by_user(&state.pool, user_id).await {
        Ok(requests) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": requests
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}

/// PATCH /api/disposal-requests/:id/status
pub async fn update_status(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(status): Json<RequestStatus>,
) -> impl IntoResponse {
    if let Err(err) = crate::utils::auth::require_admin(&auth) {
        return err.into_response();
    }

    match DisposalRequestService::update_status(&state.pool, id, status).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Status atualizado."
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

/// DELETE /api/disposal-requests/:id
pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(err) = crate::utils::auth::require_admin(&auth) {
        return err.into_response();
    }

    match DisposalRequestService::delete(&state.pool, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Solicitação removida."
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

/// GET /api/disposal-requests/:id
pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match DisposalRequestService::find_by_id(&state.pool, id).await {
        Ok(request) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": request
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}