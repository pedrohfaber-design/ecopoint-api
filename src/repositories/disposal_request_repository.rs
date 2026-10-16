use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::{
    disposal_item::DisposalItem, disposal_request::DisposalRequest, request_status::RequestStatus,
};
use crate::models::disposal_request_details::{
    DisposalRequestDetails,
    DisposalRequestItemDetails,
};

pub struct DisposalRequestRepository;

impl DisposalRequestRepository {
    /// Cria uma solicitação
    pub async fn create_request(
        tx: &mut Transaction<'_, Postgres>,
        request: &DisposalRequest,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO disposal_requests
            (
                id,
                user_id,
                collection_point_id,
                status,
                created_at,
                updated_at
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )
            "#,
            request.id,
            request.user_id,
            request.collection_point_id,
            request.status.to_string(),
            request.created_at,
            request.updated_at
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    }

    /// Adiciona um item na solicitação
    pub async fn create_item(
        tx: &mut Transaction<'_, Postgres>,
        item: &DisposalItem,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO disposal_items
            (
                id,
                request_id,
                waste_type_id,
                quantity
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4
            )
            "#,
            item.id,
            item.request_id,
            item.waste_type_id,
            item.quantity
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    }

    /// Lista todas as solicitações
    pub async fn find_all(pool: &PgPool) -> Result<Vec<DisposalRequest>, sqlx::Error> {
        let requests = sqlx::query_as!(
            DisposalRequest,
            r#"
            SELECT
                id,
                user_id,
                collection_point_id,
                status as "status: RequestStatus",
                created_at,
                updated_at
            FROM disposal_requests
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(requests)
    }

    /// Lista somente as solicitações de um usuário
pub async fn find_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<DisposalRequest>, sqlx::Error> {
    let requests = sqlx::query_as!(
        DisposalRequest,
        r#"
        SELECT
            id,
            user_id,
            collection_point_id,
            status as "status: RequestStatus",
            created_at,
            updated_at
        FROM disposal_requests
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(requests)
}

    /// Busca por ID
    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<DisposalRequest>, sqlx::Error> {
        let request = sqlx::query_as!(
            DisposalRequest,
            r#"
            SELECT
                id,
                user_id,
                collection_point_id,
                status as "status: RequestStatus",
                created_at,
                updated_at
            FROM disposal_requests
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(request)
    }

    /// Atualiza o status
    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: RequestStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE disposal_requests
            SET
                status = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            status.to_string(),
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remove uma solicitação
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM disposal_requests
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
    /// Busca uma solicitação com ecoponto e resíduos
pub async fn find_details_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<DisposalRequestDetails>, sqlx::Error> {
    let request = sqlx::query!(
        r#"
        SELECT
            dr.id,
            dr.user_id,
            dr.collection_point_id,

            cp.name AS collection_point_name,
            cp.city AS collection_point_city,
            cp.state AS collection_point_state,

            dr.status AS "status: RequestStatus",
            dr.created_at,
            dr.updated_at

        FROM disposal_requests dr

        INNER JOIN collection_points cp
            ON cp.id = dr.collection_point_id

        WHERE dr.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    let Some(request) = request else {
        return Ok(None);
    };

    let items = sqlx::query!(
        r#"
        SELECT
            di.id,
            di.waste_type_id,
            wt.name AS waste_type_name,
            di.quantity

        FROM disposal_items di

        INNER JOIN waste_types wt
            ON wt.id = di.waste_type_id

        WHERE di.request_id = $1

        ORDER BY wt.name
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    let items = items
        .into_iter()
        .map(|item| DisposalRequestItemDetails {
            id: item.id,
            waste_type_id: item.waste_type_id,
            waste_type_name: item.waste_type_name,
            quantity: item.quantity,
        })
        .collect();

    Ok(Some(DisposalRequestDetails {
        id: request.id,
        user_id: request.user_id,
        collection_point_id: request.collection_point_id,

        collection_point_name: request.collection_point_name,
        collection_point_city: request.collection_point_city,
        collection_point_state: request.collection_point_state,

        status: request.status,

        created_at: request.created_at,
        updated_at: request.updated_at,

        items,
    }))
}
}
