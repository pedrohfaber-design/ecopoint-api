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
    dto::create_chat_message::CreateChatMessageDto,
    extractors::auth_user::AuthUser,
    services::chat_message_service::ChatMessageService,
};

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(request_id): Path<Uuid>,
) -> impl IntoResponse {
    let sender_id = match Uuid::parse_str(&auth.0.sub) {
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

    let is_admin = auth.0.role.eq_ignore_ascii_case("admin");

    match ChatMessageService::list(
        &state.pool,
        request_id,
        sender_id,
        is_admin,
    )
    .await
    {
        Ok(messages) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": messages
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(request_id): Path<Uuid>,
    Json(dto): Json<CreateChatMessageDto>,
) -> impl IntoResponse {
    let sender_id = match Uuid::parse_str(&auth.0.sub) {
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

    let is_admin = auth.0.role.eq_ignore_ascii_case("admin");

    match ChatMessageService::create(
        &state.pool,
        request_id,
        sender_id,
        is_admin,
        dto.message,
    )
    .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Mensagem enviada com sucesso."
            })),
        )
            .into_response(),

        Err(err) => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        )
            .into_response(),
    }
}