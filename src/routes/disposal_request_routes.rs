use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::disposal_request_handler,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(disposal_request_handler::list)
                .post(disposal_request_handler::create),
        )
        .route(
            "/my",
            get(disposal_request_handler::my_requests),
        )
        .route(
            "/{id}",
            get(disposal_request_handler::find_by_id)
                .delete(disposal_request_handler::delete),
        )
        .route(
            "/{id}/details",
            get(disposal_request_handler::find_details_by_id),
        )
        .route(
            "/{id}/status",
            patch(disposal_request_handler::update_status),
        )
}