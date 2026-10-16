use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::user_dashboard::UserDashboardDto, models::disposal_request::DisposalRequest};

pub struct UserDashboardRepository;

impl UserDashboardRepository {
    pub async fn stats(pool: &PgPool, user_id: Uuid) -> Result<UserDashboardDto> {
        let pending: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE user_id = $1 AND status = 'Pending'",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let approved: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE user_id = $1 AND status = 'Approved'",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let completed: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE user_id = $1 AND status = 'Completed'",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let cancelled: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE user_id = $1 AND status = 'Cancelled'",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let last_requests = sqlx::query_as!(
            DisposalRequest,
            r#"
            SELECT
                id,
                user_id,
                collection_point_id,
                status as "status: _",
                created_at,
                updated_at
            FROM disposal_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 5
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(UserDashboardDto {
            pending,
            approved,
            completed,
            cancelled,
            last_requests,
        })
    }
}
