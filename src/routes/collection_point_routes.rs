use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::collection_point_handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(collection_point_handler::list).post(collection_point_handler::create),
        )
        .route(
            "/{id}",
            get(collection_point_handler::find_by_id)
                .put(collection_point_handler::update)
                .delete(collection_point_handler::delete),
        )
}
