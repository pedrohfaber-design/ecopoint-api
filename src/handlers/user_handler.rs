use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;
use crate::services::email_service::EmailService;
use crate::{
    app_state::AppState,
    dto::{
        create_user::CreateUserDto,
        forgot_password::ForgotPasswordDto,
        login_user::LoginUserDto,
        reset_password::ResetPasswordDto,
    },
    services::{
        password_reset_service::PasswordResetService,
        user_service::UserService,
    },
    utils::jwt::service::JwtService,
};

/// POST /api/users/register
pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> impl IntoResponse {
    match UserService::register(&state.pool, dto).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Usuário cadastrado com sucesso.",
                "data": {
                    "id": user.id,
                    "name": user.name,
                    "email": user.email,
                    "role": user.role,
                    "created_at": user.created_at
                }
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

/// POST /api/users/login
pub async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginUserDto>,
) -> impl IntoResponse {
    match UserService::login(&state.pool, dto).await {
        Ok(user) => {
            let token = JwtService::generate(
                &user.id.to_string(),
                &format!("{:?}", user.role),
                &state.settings,
            )
            .unwrap();

            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Login realizado com sucesso.",
                    "token": token,
                    "user": {
                        "id": user.id,
                        "name": user.name,
                        "email": user.email,
                        "role": user.role
                    }
                })),
            )
        }

        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        ),
    }
}

/// POST /api/users/test-email
pub async fn test_email() -> impl IntoResponse {
    match EmailService::send(
        "pedrohfaber@gmail.com",
        "Teste EcoPoint",
        "Este é um teste de envio de e-mail do sistema EcoPoint.",
    )
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "E-mail enviado com sucesso."
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

/// GET /api/users
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match UserService::list(&state.pool).await {
        Ok(users) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": users
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

/// GET /api/users/:id
pub async fn find_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match UserService::find_by_id(&state.pool, id).await {
        Ok(user) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": user
            })),
        ),

        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": err.to_string()
            })),
        ),
    }
}

/// DELETE /api/users/:id
pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match UserService::delete(&state.pool, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Usuário removido com sucesso."
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

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(dto): Json<ForgotPasswordDto>,
) -> impl IntoResponse {
    match PasswordResetService::create_reset_token(
        &state.pool,
        &dto.email,
    )
    .await
    {
        Ok(Some(_token)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Se o e-mail estiver cadastrado, enviaremos as instruções para recuperação."
            })),
        ),

        Ok(None) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Se o e-mail estiver cadastrado, enviaremos as instruções para recuperação."
            })),
        ),

        Err(err) => {
            eprintln!("ERRO FORGOT PASSWORD: {:?}", err);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": format!("Erro interno: {}", err)
                })),
            )
        }
    }
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(dto): Json<ResetPasswordDto>,
) -> impl IntoResponse {
    match PasswordResetService::reset_password(
        &state.pool,
        &dto.token,
        &dto.password,
        &dto.password_confirmation,
    )
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Senha redefinida com sucesso."
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