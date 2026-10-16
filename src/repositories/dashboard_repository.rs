use anyhow::Result;
use sqlx::PgPool;

use crate::dto::dashboard_stats::DashboardStatsDto;

pub struct DashboardRepository;

impl DashboardRepository {
    pub async fn stats(pool: &PgPool) -> Result<DashboardStatsDto> {
        let total_users: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

        let total_collection_points: i64 =
            sqlx::query_scalar!("SELECT COUNT(*) FROM collection_points")
                .fetch_one(pool)
                .await?
                .unwrap_or(0);

        let total_waste_types: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM waste_types")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

        let pending_requests: i64 =
            sqlx::query_scalar!("SELECT COUNT(*) FROM disposal_requests WHERE status = 'Pending'")
                .fetch_one(pool)
                .await?
                .unwrap_or(0);

        let approved_requests: i64 =
            sqlx::query_scalar!("SELECT COUNT(*) FROM disposal_requests WHERE status = 'Approved'")
                .fetch_one(pool)
                .await?
                .unwrap_or(0);

        let completed_requests: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE status = 'Completed'"
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let cancelled_requests: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM disposal_requests WHERE status = 'Cancelled'"
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok(DashboardStatsDto {
            total_users,
            total_collection_points,
            total_waste_types,
            pending_requests,
            approved_requests,
            completed_requests,
            cancelled_requests,
        })
    }
}
