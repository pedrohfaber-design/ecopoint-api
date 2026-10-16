use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::config::settings::Settings;

/// Cria um pool de conexões com o PostgreSQL.
pub async fn create_pool(settings: &Settings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        // Número máximo de conexões simultâneas
        .max_connections(10)
        // Número mínimo de conexões abertas
        .min_connections(2)
        // Tempo máximo para estabelecer conexão
        .acquire_timeout(Duration::from_secs(10))
        // Conecta ao banco
        .connect(&settings.database_url)
        .await
}

/// Testa a conexão com o banco de dados.
///
/// Será executado quando a aplicação iniciar.
pub async fn test_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;

    println!("✅ PostgreSQL conectado com sucesso!");

    Ok(())
}
