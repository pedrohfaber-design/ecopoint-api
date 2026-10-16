use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::chat_message::ChatMessage,
    repositories::{
        chat_message_repository::ChatMessageRepository,
        disposal_request_repository::DisposalRequestRepository,
    },
};

pub struct ChatMessageService;

impl ChatMessageService {
    pub async fn create(
        pool: &PgPool,
        request_id: Uuid,
        sender_id: Uuid,
        is_admin: bool,
        message: String,
    ) -> Result<()> {
        let request =
            DisposalRequestRepository::find_by_id(pool, request_id)
                .await?
                .ok_or_else(|| anyhow!("Solicitação não encontrada"))?;

        if request.user_id != sender_id && !is_admin {
            return Err(anyhow!(
                "Usuário não autorizado a enviar mensagens nesta solicitação"
            ));
        }

        let message = message.trim();

        if message.is_empty() {
            return Err(anyhow!(
                "A mensagem não pode estar vazia"
            ));
        }

        let chat_message = ChatMessage {
    id: Uuid::new_v4(),
    request_id,
    sender_id,
    sender_name: String::new(),
    message: message.to_string(),
    created_at: Utc::now(),
};

        ChatMessageRepository::create(pool, &chat_message).await?;

        Ok(())
    }

    pub async fn list(
        pool: &PgPool,
        request_id: Uuid,
        sender_id: Uuid,
        is_admin: bool,
    ) -> Result<Vec<ChatMessage>> {
        let request =
            DisposalRequestRepository::find_by_id(pool, request_id)
                .await?
                .ok_or_else(|| anyhow!("Solicitação não encontrada"))?;

        if request.user_id != sender_id && !is_admin {
            return Err(anyhow!(
                "Usuário não autorizado a visualizar esta conversa"
            ));
        }

        Ok(
            ChatMessageRepository::find_by_request(pool, request_id)
                .await?
        )
    }
}