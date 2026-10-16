use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            name,
            email,
            password_hash,
            phone,
            city,
            state,
            role,
            created_at,
            updated_at
        FROM users
        WHERE LOWER(email) = LOWER($1)
        "#,
    )
    .bind(email.trim())
    .fetch_optional(pool)
    .await
}

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                name,
                email,
                password_hash,
                phone,
                city,
                state,
                role,
                created_at,
                updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users
            (
                id,
                name,
                email,
                password_hash,
                phone,
                city,
                state,
                role,
                created_at,
                updated_at
            )
            VALUES
            (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10
            )
            "#,
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.phone)
        .bind(&user.city)
        .bind(&user.state)
        .bind(format!("{:?}", user.role))
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                name,
                email,
                password_hash,
                phone,
                city,
                state,
                role,
                created_at,
                updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
    
    pub async fn update_password(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET
            password_hash = $1,
            updated_at = NOW()
        WHERE id = $2
        "#,
        password_hash,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
}
