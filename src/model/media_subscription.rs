use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{Iden, OnConflict, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::DbError;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "media_subscriptions")]
pub enum MediaSubscriptionIden {
    Table,
    MediaSubscriptionId,
    BuyerUserId,
    OfferId,
    CurrentPeriodStart,
    CurrentPeriodEnd,
    SubscriptionStatus,
    PayedAt,
    PayedUntil,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone)]
pub struct MediaSubscription {
    pub media_subscription_id: Uuid,
    pub buyer_user_id: String,
    pub offer_id: Uuid,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub subscription_status: String,
    pub payed_at: DateTime<Utc>,
    pub payed_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MediaSubscription {
    pub const ACTIVE_KEY: &'static str = "active";

    const PUT_COLUMNS: [MediaSubscriptionIden; 8] = [
        MediaSubscriptionIden::MediaSubscriptionId,
        MediaSubscriptionIden::BuyerUserId,
        MediaSubscriptionIden::OfferId,
        MediaSubscriptionIden::CurrentPeriodStart,
        MediaSubscriptionIden::CurrentPeriodEnd,
        MediaSubscriptionIden::SubscriptionStatus,
        MediaSubscriptionIden::PayedAt,
        MediaSubscriptionIden::PayedUntil,
    ];

    #[allow(clippy::too_many_arguments)]
    pub async fn put(
        pool: &Pool,
        media_subscription_id: &Uuid,
        buyer_user_id: &String,
        offer_id: &Uuid,
        current_period_start: &DateTime<Utc>,
        current_period_end: &DateTime<Utc>,
        subscription_status: &String,
        payed_at: &DateTime<Utc>,
        payed_until: &DateTime<Utc>,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaSubscriptionIden::Table)
            .columns(Self::PUT_COLUMNS)
            .values([
                (*media_subscription_id).into(),
                buyer_user_id.into(),
                (*offer_id).into(),
                (*current_period_start).into(),
                (*current_period_end).into(),
                subscription_status.into(),
                (*payed_at).into(),
                (*payed_until).into(),
            ])?
            .on_conflict(
                OnConflict::column(MediaSubscriptionIden::MediaSubscriptionId)
                    .update_columns(Self::PUT_COLUMNS)
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = conn
            .query_one(sql.as_str(), values.as_params().as_ref())
            .await?;

        Ok(Self::from(row))
    }
}

impl From<Row> for MediaSubscription {
    fn from(row: Row) -> Self {
        Self {
            media_subscription_id: row.get(
                MediaSubscriptionIden::MediaSubscriptionId
                    .to_string()
                    .as_str(),
            ),
            buyer_user_id: row
                .get(MediaSubscriptionIden::BuyerUserId.to_string().as_str()),
            offer_id: row
                .get(MediaSubscriptionIden::OfferId.to_string().as_str()),
            current_period_start: row.get(
                MediaSubscriptionIden::CurrentPeriodStart
                    .to_string()
                    .as_str(),
            ),
            current_period_end: row.get(
                MediaSubscriptionIden::CurrentPeriodEnd.to_string().as_str(),
            ),
            subscription_status: row.get(
                MediaSubscriptionIden::SubscriptionStatus
                    .to_string()
                    .as_str(),
            ),
            payed_at: row
                .get(MediaSubscriptionIden::PayedAt.to_string().as_str()),
            payed_until: row
                .get(MediaSubscriptionIden::PayedUntil.to_string().as_str()),
            created_at: row
                .get(MediaSubscriptionIden::CreatedAt.to_string().as_str()),
            updated_at: row
                .get(MediaSubscriptionIden::UpdatedAt.to_string().as_str()),
        }
    }
}
