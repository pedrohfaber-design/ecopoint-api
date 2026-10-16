use axum::{routing::get, Router};

use crate::{
    app_state::AppState,
    handlers::chat_message_handler,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/{request_id}",
            get(chat_message_handler::list)
                .post(chat_message_handler::create),
        )
}