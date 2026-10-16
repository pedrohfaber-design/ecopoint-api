use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat_message::ChatMessage;

pub struct ChatMessageRepository;

impl ChatMessageRepository {
    pub async fn create(
        pool: &PgPool,
        message: &ChatMessage,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO chat_messages (
                id,
                request_id,
                sender_id,
                message,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            message.id,
            message.request_id,
            message.sender_id,
            message.message,
            message.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

   pub async fn find_by_request(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<Vec<ChatMessage>, sqlx::Error> {
    let messages = sqlx::query_as!(
        ChatMessage,
        r#"
        SELECT
            cm.id,
            cm.request_id,
            cm.sender_id,
            u.name AS sender_name,
            cm.message,
            cm.created_at
        FROM chat_messages cm
        INNER JOIN users u
            ON u.id = cm.sender_id
        WHERE cm.request_id = $1
        ORDER BY cm.created_at ASC
        "#,
        request_id
    )
    .fetch_all(pool)
    .await?;

    Ok(messages)
}
}