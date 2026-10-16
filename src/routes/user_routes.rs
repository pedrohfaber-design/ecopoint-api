use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::handlers::user_handler;

pub fn router() -> Router<crate::app_state::AppState> {
    Router::new()
        .route("/register", post(user_handler::register))
        .route("/login", post(user_handler::login))
        .route("/", get(user_handler::list))
        .route("/{id}", get(user_handler::find_by_id))
        .route("/{id}", delete(user_handler::delete))
        .route("/forgot-password", post(user_handler::forgot_password))
        .route("/reset-password", post(user_handler::reset_password))
        .route(
    "/test-email",
    post(user_handler::test_email),
)
}