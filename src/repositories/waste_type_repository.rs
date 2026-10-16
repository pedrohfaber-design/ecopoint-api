use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::{create_waste_type::CreateWasteTypeDto, update_waste_type::UpdateWasteTypeDto},
    models::waste_type::WasteType,
};

pub struct WasteTypeRepository;

impl WasteTypeRepository {
    pub async fn create(pool: &PgPool, dto: &CreateWasteTypeDto) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO waste_types (
                id,
                name,
                description
            )
            VALUES ($1, $2, $3)
            "#,
            Uuid::new_v4(),
            dto.name,
            dto.description,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<WasteType>> {
        let waste_types = sqlx::query_as!(
            WasteType,
            r#"
            SELECT
                id,
                name,
                description
            FROM waste_types
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(waste_types)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<WasteType>> {
        let waste_type = sqlx::query_as!(
            WasteType,
            r#"
            SELECT
                id,
                name,
                description
            FROM waste_types
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(waste_type)
    }

    pub async fn update(pool: &PgPool, id: Uuid, dto: &UpdateWasteTypeDto) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE waste_types
            SET
                name = $1,
                description = $2
            WHERE id = $3
            "#,
            dto.name,
            dto.description,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM waste_types
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
