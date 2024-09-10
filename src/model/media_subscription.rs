use std::convert::identity;

use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{
    any, Asterisk, Expr, Iden, OnConflict, PostgresQueryBuilder, Query,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::{get_count_from_rows, DbError};

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
    CancelAt,
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
    pub cancel_at: Option<DateTime<Utc>>,
}

impl MediaSubscription {
    const PUT_COLUMNS: [MediaSubscriptionIden; 12] = [
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
        MediaSubscriptionIden::CancelAt,
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
        cancel_at: Option<DateTime<Utc>>,
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
                cancel_at.into(),
            ])?
            .on_conflict(
                OnConflict::column(MediaSubscriptionIden::MediaSubscriptionId)
                    .update_columns(Self::PUT_COLUMNS)
                    .to_owned(),
            )
            .on_conflict(
                OnConflict::columns([
                    MediaSubscriptionIden::BuyerUserId,
                    MediaSubscriptionIden::OfferId,
                ])
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

    pub async fn upsert(
        pool: &Pool,
        media_subscription: MediaSubscription,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaSubscriptionIden::Table)
            .columns(Self::PUT_COLUMNS)
            .values([
                media_subscription.media_subscription_id.into(),
                media_subscription.buyer_user_id.into(),
                media_subscription.offer_id.into(),
                media_subscription.shop_id.into(),
                media_subscription.current_period_start.into(),
                media_subscription.current_period_end.into(),
                media_subscription.subscription_status.into(),
                media_subscription.payed_at.into(),
                media_subscription.payed_until.into(),
                media_subscription.stripe_subscription_id.into(),
                media_subscription.canceled_at.into(),
                media_subscription.cancel_at.into(),
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
                .cond_where(
                    Expr::col(MediaSubscriptionIden::BuyerUserId)
                        .eq(buyer_user_id),
                )
                .cond_where(any![
                    Expr::col(MediaSubscriptionIden::SubscriptionStatus)
                        .eq("active"),
                    Expr::col(MediaSubscriptionIden::SubscriptionStatus)
                        .eq("trialing")
                ]);

            if let Some(media_subscription_id) = media_subscription_id {
                query.cond_where(
                    Expr::col(MediaSubscriptionIden::MediaSubscriptionId)
                        .eq(media_subscription_id),
                );
            }

            if let Some(offer_id) = offer_id {
                query.cond_where(
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
        is_accessible: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<Self>, i64), DbError> {
        let mut conn = pool.get().await?;
        let transaction = conn.transaction().await?;

        let ((sql, values), (count_sql, count_values)) = {
            let mut query = Query::select();

            query
                .from(MediaSubscriptionIden::Table)
                .cond_where(
                    Expr::col(MediaSubscriptionIden::BuyerUserId)
                        .eq(buyer_user_id),
                )
                .cond_where(any![
                    Expr::col(MediaSubscriptionIden::SubscriptionStatus)
                        .eq("active"),
                    Expr::col(MediaSubscriptionIden::SubscriptionStatus)
                        .eq("trialing")
                ]);

            if let Some(shop_id) = shop_id {
                query.cond_where(
                    Expr::col(MediaSubscriptionIden::ShopId).eq(shop_id),
                );
            }

            if is_accessible.is_some_and(identity) {
                query.cond_where(
                    Expr::col((
                        MediaSubscriptionIden::Table,
                        MediaSubscriptionIden::PayedUntil,
                    ))
                    .gte(Utc::now()),
                );
            }

            (
                query
                    .clone()
                    .column(Asterisk)
                    .limit(limit)
                    .offset(offset)
                    .build_postgres(PostgresQueryBuilder),
                query
                    .expr(
                        Expr::col((MediaSubscriptionIden::Table, Asterisk))
                            .count(),
                    )
                    .build_postgres(PostgresQueryBuilder),
            )
        };

        let rows = transaction.query(sql.as_str(), &values.as_params()).await?;
        let count_rows = transaction
            .query(count_sql.as_str(), &count_values.as_params())
            .await?;
        transaction.commit().await?;

        let count = get_count_from_rows(&count_rows);

        Ok((rows.iter().map(Self::from).collect(), count))
    }

    pub async fn delete(
        pool: &Pool,
        media_subscription_id: &Uuid,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(MediaSubscriptionIden::Table)
            .and_where(
                Expr::col(MediaSubscriptionIden::MediaSubscriptionId)
                    .eq(*media_subscription_id),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_one(sql.as_str(), &values.as_params())
            .await?
            .into())
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
            cancel_at: row
                .get(MediaSubscriptionIden::CancelAt.to_string().as_str()),
        }
    }
}

impl From<Row> for MediaSubscription {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
