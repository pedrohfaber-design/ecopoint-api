use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::user_dashboard_handler};

pub fn router() -> Router<AppState> {
    Router::new().route("/{user_id}", get(user_dashboard_handler::stats))
}
