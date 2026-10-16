use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};

use crate::{
    app_state::AppState,
    utils::jwt::{
        claims::Claims,
        service::JwtService,
    },
};

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Authorization não informado"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Token inválido"))?;

        let claims = JwtService::verify(token, &state.settings)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Token inválido"))?;

        Ok(AuthUser(claims))
    }
}
