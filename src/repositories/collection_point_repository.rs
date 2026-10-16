use sqlx::PgPool;
use uuid::Uuid;

use crate::models::collection_point::CollectionPoint;

pub struct CollectionPointRepository;

impl CollectionPointRepository {
    /// Lista todos os pontos de coleta
    pub async fn find_all(pool: &PgPool) -> Result<Vec<CollectionPoint>, sqlx::Error> {
        let points = sqlx::query_as!(
            CollectionPoint,
            r#"
            SELECT
                id,
                name,
                description,
                address,
                city,
                state,
                latitude,
                longitude,
                phone,
                email,
                created_at,
                updated_at
            FROM collection_points
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(points)
    }

    /// Busca um ponto pelo ID
    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<CollectionPoint>, sqlx::Error> {
        let point = sqlx::query_as!(
            CollectionPoint,
            r#"
            SELECT
                id,
                name,
                description,
                address,
                city,
                state,
                latitude,
                longitude,
                phone,
                email,
                created_at,
                updated_at
            FROM collection_points
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(point)
    }

    /// Cria um novo ponto
    pub async fn create(pool: &PgPool, point: &CollectionPoint) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO collection_points
            (
                id,
                name,
                description,
                address,
                city,
                state,
                latitude,
                longitude,
                phone,
                email,
                created_at,
                updated_at
            )
            VALUES
            (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12
            )
            "#,
            point.id,
            point.name,
            point.description,
            point.address,
            point.city,
            point.state,
            point.latitude,
            point.longitude,
            point.phone,
            point.email,
            point.created_at,
            point.updated_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remove um ponto
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM collection_points
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(pool: &PgPool, point: &CollectionPoint) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        UPDATE collection_points
        SET
            name = $1,
            description = $2,
            address = $3,
            city = $4,
            state = $5,
            latitude = $6,
            longitude = $7,
            phone = $8,
            email = $9,
            updated_at = NOW()
        WHERE id = $10
        "#,
            point.name,
            point.description,
            point.address,
            point.city,
            point.state,
            point.latitude,
            point.longitude,
            point.phone,
            point.email,
            point.id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
