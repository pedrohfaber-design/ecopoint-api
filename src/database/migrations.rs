use std::{collections::HashSet, fs, path::Path};

use anyhow::Result;
use sqlx::{raw_sql, PgPool, Row};
use tracing::info;

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("==============================");
    info!("Verificando migrations...");
    info!("==============================");

    // Cria a tabela de controle, caso não exista
    ensure_migrations_table(pool).await?;

    // Busca quais migrations já foram executadas
    let executed = get_executed_migrations(pool).await?;

    // Executa apenas as pendentes
    execute_pending_migrations(pool, &executed).await?;

    info!("Todas as migrations estão atualizadas.");

    Ok(())
}
async fn ensure_migrations_table(pool: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _schema_migrations (
            id SERIAL PRIMARY KEY,
            filename VARCHAR(255) NOT NULL UNIQUE,
            executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
async fn get_executed_migrations(pool: &PgPool) -> Result<HashSet<String>> {
    let rows = sqlx::query(
        r#"
        SELECT filename
        FROM _schema_migrations
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut executed = HashSet::new();

    for row in rows {
        let filename: String = row.get("filename");
        executed.insert(filename);
    }

    Ok(executed)
}
async fn execute_pending_migrations(pool: &PgPool, executed: &HashSet<String>) -> Result<()> {
    let migrations_path = Path::new("migrations");

    if !migrations_path.exists() {
        info!("Pasta migrations não encontrada.");
        return Ok(());
    }

    let mut files = fs::read_dir(migrations_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    files.sort_by_key(|f| {
    let filename = f.file_name().to_string_lossy().to_string();

    let number = filename
        .split('_')
        .next()
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    (number, filename)
});

    for file in files {
        let filename = file.file_name().to_string_lossy().to_string();

        if executed.contains(&filename) {
            info!("Migration já executada: {}", filename);
            continue;
        }

        let sql = fs::read_to_string(file.path())?;

        info!("Executando migration: {}", filename);

        raw_sql(&sql).execute(pool).await?;

        mark_as_executed(pool, &filename).await?;

        info!("Migration concluída: {}", filename);
    }

    Ok(())
}
async fn mark_as_executed(pool: &PgPool, filename: &str) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO _schema_migrations (filename)
        VALUES ($1)
        "#,
    )
    .bind(filename)
    .execute(pool)
    .await?;

    Ok(())
}
