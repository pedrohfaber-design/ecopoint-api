use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::waste_type_handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(waste_type_handler::list).post(waste_type_handler::create),
        )
        .route(
            "/{id}",
            get(waste_type_handler::find_by_id)
                .put(waste_type_handler::update)
                .delete(waste_type_handler::delete),
        )
}
