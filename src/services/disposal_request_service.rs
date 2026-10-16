use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::create_disposal_request::CreateDisposalRequestDto,
    models::{
        disposal_item::DisposalItem, disposal_request::DisposalRequest,
        request_status::RequestStatus,
    },
    repositories::disposal_request_repository::DisposalRequestRepository,
};
use crate::models::disposal_request_details::DisposalRequestDetails;

pub struct DisposalRequestService;

impl DisposalRequestService {
    /// Cria uma solicitação com todos os itens
    pub async fn create(pool: &PgPool, dto: CreateDisposalRequestDto) -> Result<()> {
        let mut tx = pool.begin().await?;

        let request_id = Uuid::new_v4();

        let request = DisposalRequest {
            id: request_id,
            user_id: dto.user_id,
            collection_point_id: dto.collection_point_id,
            status: RequestStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        DisposalRequestRepository::create_request(&mut tx, &request).await?;

        for item in dto.items {
            let disposal_item = DisposalItem {
                id: Uuid::new_v4(),
                request_id,
                waste_type_id: item.waste_type_id,
                quantity: item.quantity,
            };

            DisposalRequestRepository::create_item(&mut tx, &disposal_item).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    /// Lista todas as solicitações
    pub async fn list(pool: &PgPool) -> Result<Vec<DisposalRequest>> {
        Ok(DisposalRequestRepository::find_all(pool).await?)
    }
    /// Lista as solicitações de um usuário
pub async fn list_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<DisposalRequest>> {
    Ok(
        DisposalRequestRepository::find_by_user_id(pool, user_id)
            .await?
    )
}

    /// Busca uma solicitação
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<DisposalRequest> {
        Ok(DisposalRequestRepository::find_by_id(pool, id)
            .await?
            .expect("Solicitação não encontrada"))
    }

    /// Atualiza o status
    pub async fn update_status(pool: &PgPool, id: Uuid, status: RequestStatus) -> Result<()> {
        DisposalRequestRepository::update_status(pool, id, status).await?;

        Ok(())
    }

    /// Remove uma solicitação
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        DisposalRequestRepository::delete(pool, id).await?;

        Ok(())
    }
    /// Busca uma solicitação com todos os detalhes
pub async fn find_details_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<DisposalRequestDetails>> {
    Ok(
        DisposalRequestRepository::find_details_by_id(pool, id)
            .await?
    )
}
}
