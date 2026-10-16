use axum::http::StatusCode;

use crate::extractors::auth_user::AuthUser;

pub fn require_admin(auth: &AuthUser) -> Result<(), (StatusCode, &'static str)> {
    if auth.0.role.eq_ignore_ascii_case("admin") {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            "Apenas administradores podem acessar este recurso.",
        ))
    }
}
