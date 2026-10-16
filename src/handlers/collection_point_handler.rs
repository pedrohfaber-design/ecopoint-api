use crate::dto::update_collection_point::UpdateCollectionPointDto;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_state::AppState, dto::create_collection_point::CreateCollectionPointDto,
    models::collection_point::CollectionPoint,
    repositories::collection_point_repository::CollectionPointRepository,
};

use crate::{extractors::auth_user::AuthUser, utils::auth::require_admin};

/// GET /api/collection-points
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match CollectionPointRepository::find_all(&state.pool).await {
        Ok(points) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": points
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

/// GET /api/collection-points/:id
pub async fn find_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match CollectionPointRepository::find_by_id(&state.pool, id).await {
        Ok(Some(point)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": point
            })),
        ),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Ponto não encontrado."
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

/// POST /api/collection-points
pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(dto): Json<CreateCollectionPointDto>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }
    let point = CollectionPoint {
        id: Uuid::new_v4(),

        name: dto.name,

        description: dto.description,

        address: dto.address,

        city: dto.city,

        state: dto.state,

        latitude: dto.latitude,

        longitude: dto.longitude,

        phone: dto.phone,

        email: dto.email,

        created_at: Utc::now(),

        updated_at: Utc::now(),
    };

    match CollectionPointRepository::create(&state.pool, &point).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Ponto criado com sucesso."
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

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateCollectionPointDto>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }

    let mut point = match CollectionPointRepository::find_by_id(&state.pool, id).await {
        Ok(Some(point)) => point,

        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "message": "Ponto de coleta não encontrado."
                })),
            )
                .into_response();
        }

        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": err.to_string()
                })),
            )
                .into_response();
        }
    };

    if let Some(name) = dto.name {
        point.name = name;
    }

    if let Some(description) = dto.description {
        point.description = description;
    }

    if let Some(address) = dto.address {
        point.address = address;
    }

    if let Some(city) = dto.city {
        point.city = city;
    }

    if let Some(state_name) = dto.state {
        point.state = state_name;
    }

    if let Some(latitude) = dto.latitude {
        point.latitude = latitude;
    }

    if let Some(longitude) = dto.longitude {
        point.longitude = longitude;
    }

    if let Some(phone) = dto.phone {
        point.phone = Some(phone);
    }

    if let Some(email) = dto.email {
        point.email = Some(email);
    }

    point.updated_at = Utc::now();

    match CollectionPointRepository::update(&state.pool, &point).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Ponto atualizado com sucesso."
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

/// DELETE /api/collection-points/:id
pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(err) = require_admin(&auth) {
        return err.into_response();
    }

    match CollectionPointRepository::delete(&state.pool, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Ponto removido."
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
