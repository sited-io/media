use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{
    Asterisk, Expr, Iden, OnConflict, PostgresQueryBuilder, Query,
};
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
    ShopId,
    CurrentPeriodStart,
    CurrentPeriodEnd,
    SubscriptionStatus,
    PayedAt,
    PayedUntil,
    CreatedAt,
    UpdatedAt,
    StripeSubscriptionId,
    CanceledAt,
}

#[derive(Debug, Clone)]
pub struct MediaSubscription {
    pub media_subscription_id: Uuid,
    pub buyer_user_id: String,
    pub offer_id: Uuid,
    pub shop_id: Uuid,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub subscription_status: String,
    pub payed_at: DateTime<Utc>,
    pub payed_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stripe_subscription_id: Option<String>,
    pub canceled_at: Option<DateTime<Utc>>,
}

impl MediaSubscription {
    const PUT_COLUMNS: [MediaSubscriptionIden; 11] = [
        MediaSubscriptionIden::MediaSubscriptionId,
        MediaSubscriptionIden::BuyerUserId,
        MediaSubscriptionIden::OfferId,
        MediaSubscriptionIden::ShopId,
        MediaSubscriptionIden::CurrentPeriodStart,
        MediaSubscriptionIden::CurrentPeriodEnd,
        MediaSubscriptionIden::SubscriptionStatus,
        MediaSubscriptionIden::PayedAt,
        MediaSubscriptionIden::PayedUntil,
        MediaSubscriptionIden::StripeSubscriptionId,
        MediaSubscriptionIden::CanceledAt,
    ];

    #[allow(clippy::too_many_arguments)]
    pub async fn put(
        pool: &Pool,
        media_subscription_id: &Uuid,
        buyer_user_id: &String,
        offer_id: &Uuid,
        shop_id: &Uuid,
        current_period_start: &DateTime<Utc>,
        current_period_end: &DateTime<Utc>,
        subscription_status: &String,
        payed_at: &DateTime<Utc>,
        payed_until: &DateTime<Utc>,
        stripe_subscription_id: Option<String>,
        canceled_at: Option<DateTime<Utc>>,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaSubscriptionIden::Table)
            .columns(Self::PUT_COLUMNS)
            .values([
                (*media_subscription_id).into(),
                buyer_user_id.into(),
                (*offer_id).into(),
                (*shop_id).into(),
                (*current_period_start).into(),
                (*current_period_end).into(),
                subscription_status.into(),
                (*payed_at).into(),
                (*payed_until).into(),
                stripe_subscription_id.into(),
                canceled_at.into(),
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

    pub async fn get(
        pool: &Pool,
        buyer_user_id: &String,
        media_subscription_id: Option<Uuid>,
        offer_id: Option<Uuid>,
    ) -> Result<Option<Self>, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::select();

            query
                .column(Asterisk)
                .from(MediaSubscriptionIden::Table)
                .and_where(
                    Expr::col(MediaSubscriptionIden::BuyerUserId)
                        .eq(buyer_user_id),
                );

            if let Some(media_subscription_id) = media_subscription_id {
                query.and_where(
                    Expr::col(MediaSubscriptionIden::MediaSubscriptionId)
                        .eq(media_subscription_id),
                );
            }

            if let Some(offer_id) = offer_id {
                query.and_where(
                    Expr::col(MediaSubscriptionIden::OfferId).eq(offer_id),
                );
            }

            query.build_postgres(PostgresQueryBuilder)
        };

        let row = conn.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }

    pub async fn list(
        pool: &Pool,
        buyer_user_id: &String,
        shop_id: Option<Uuid>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::select();

            query
                .column(Asterisk)
                .from(MediaSubscriptionIden::Table)
                .and_where(
                    Expr::col(MediaSubscriptionIden::BuyerUserId)
                        .eq(buyer_user_id),
                );

            if let Some(shop_id) = shop_id {
                query.and_where(
                    Expr::col(MediaSubscriptionIden::ShopId).eq(shop_id),
                );
            }

            query
                .limit(limit)
                .offset(offset)
                .build_postgres(PostgresQueryBuilder)
        };

        let rows = conn.query(sql.as_str(), &values.as_params()).await?;

        Ok(rows.iter().map(Self::from).collect())
    }
}

impl From<&Row> for MediaSubscription {
    fn from(row: &Row) -> Self {
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
            shop_id: row
                .get(MediaSubscriptionIden::ShopId.to_string().as_str()),
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
            stripe_subscription_id: row.get(
                MediaSubscriptionIden::StripeSubscriptionId
                    .to_string()
                    .as_str(),
            ),
            canceled_at: row
                .get(MediaSubscriptionIden::CanceledAt.to_string().as_str()),
        }
    }
}

impl From<Row> for MediaSubscription {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
