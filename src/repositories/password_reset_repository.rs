use sqlx::PgPool;
use uuid::Uuid;

pub struct PasswordResetRepository;

impl PasswordResetRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO password_reset_tokens
                (user_id, token, expires_at)
            VALUES
                ($1, $2, $3)
            "#,
            user_id,
            token,
            expires_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_valid_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<(Uuid, Uuid)>, sqlx::Error> {
        sqlx::query_as::<_, (Uuid, Uuid)>(
            r#"
            SELECT id, user_id
            FROM password_reset_tokens
            WHERE token = $1
              AND used = FALSE
              AND expires_at > NOW()
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    pub async fn mark_as_used(
        pool: &PgPool,
        token: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE password_reset_tokens
            SET used = TRUE
            WHERE token = $1
            "#,
            token
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}