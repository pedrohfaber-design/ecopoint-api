use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::home};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(home::index))
}
