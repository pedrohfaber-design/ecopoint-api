use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::dashboard_handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stats", get(dashboard_handler::stats))
        .route("/me", get(dashboard_handler::me))
}
