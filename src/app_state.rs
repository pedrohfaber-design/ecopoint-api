use sqlx::PgPool;
use std::sync::Arc;

use crate::config::settings::Settings;

/// Estado compartilhado da aplicação.
///
/// Tudo que precisa ser acessado pelos handlers
/// ficará armazenado aqui.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Arc<Settings>,
}

impl AppState {
    pub fn new(pool: PgPool, settings: Settings) -> Self {
        Self {
            pool,
            settings: Arc::new(settings),
        }
    }
}
