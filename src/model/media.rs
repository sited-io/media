use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::{Pool, Transaction};
use sea_query::{
    Alias, Asterisk, Expr, Iden, IntoColumnRef, Order, PostgresQueryBuilder,
    Query, SelectStatement,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::api::sited_io::media::v1::{MediaFilterField, MediaOrderByField};
use crate::api::sited_io::types::v1::Direction;
use crate::db::{get_count_from_rows, DbError};

use super::media_offer::{MediaOfferIden, MediaOffersVec};
use super::media_subscription::MediaSubscriptionIden;
use super::MediaOffer;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "medias")]
pub enum MediaIden {
    Table,
    MediaId,
    ShopId,
    UserId,
    CreatedAt,
    UpdatedAt,
    Name,
    DataUrl,
    SizeBytes,
    FileName,
}

#[derive(Debug, Clone)]
pub struct Media {
    pub media_id: Uuid,
    pub offer_ids: Option<Vec<Uuid>>,
    pub shop_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub data_url: String,
    pub size_bytes: u64,
    pub file_name: String,
    pub ordering: i64,
}

impl Media {
    const MEDIA_OFFERS_ALIAS: &'static str = "offers";

    fn get_media_offers_alias() -> Alias {
        Alias::new(Self::MEDIA_OFFERS_ALIAS)
    }

    fn select_with_offer_ids() -> SelectStatement {
        Query::select()
            .expr_as(MediaOffer::get_agg(), Self::get_media_offers_alias())
            .from(MediaIden::Table)
            .left_join(
                MediaOfferIden::Table,
                Expr::col((MediaIden::Table, MediaIden::MediaId))
                    .equals((MediaOfferIden::Table, MediaOfferIden::MediaId)),
            )
            .group_by_columns([
                (MediaIden::Table, MediaIden::MediaId).into_column_ref(),
                (MediaOfferIden::Table, MediaOfferIden::Ordering)
                    .into_column_ref(),
            ])
            .to_owned()
    }

    fn select_count() -> SelectStatement {
        Query::select()
            .expr(Expr::col((MediaIden::Table, Asterisk)).count())
            .from(MediaIden::Table)
            .left_join(
                MediaOfferIden::Table,
                Expr::col((MediaIden::Table, MediaIden::MediaId))
                    .equals((MediaOfferIden::Table, MediaOfferIden::MediaId)),
            )
            .to_owned()
    }

    fn select_accessible(user_id: &String) -> SelectStatement {
        Query::select()
            .from(MediaIden::Table)
            .left_join(
                MediaOfferIden::Table,
                Expr::col((MediaIden::Table, MediaIden::MediaId))
                    .equals((MediaOfferIden::Table, MediaOfferIden::MediaId)),
            )
            .left_join(
                MediaSubscriptionIden::Table,
                Expr::col((MediaOfferIden::Table, MediaOfferIden::OfferId))
                    .equals((
                        MediaSubscriptionIden::Table,
                        MediaSubscriptionIden::OfferId,
                    )),
            )
            .and_where(
                Expr::col((
                    MediaSubscriptionIden::Table,
                    MediaSubscriptionIden::BuyerUserId,
                ))
                .eq(user_id),
            )
            .and_where(
                Expr::col((
                    MediaSubscriptionIden::Table,
                    MediaSubscriptionIden::PayedUntil,
                ))
                .gte(Utc::now()),
            )
            .to_owned()
    }

    fn add_filter(
        query: &mut SelectStatement,
        filter_field: MediaFilterField,
        filter_query: String,
    ) -> Result<(), DbError> {
        use MediaFilterField::*;

        match filter_field {
            Unspecified => Ok(()),
            Name => {
                query.and_where(
                    Expr::col((MediaIden::Table, MediaIden::Name))
                        .eq(filter_query),
                );
                Ok(())
            }
            OfferId => {
                let offer_id: Uuid = filter_query
                    .parse()
                    .map_err(|err| DbError::Other(Some(format!("{}", err))))?;
                query.and_where(
                    Expr::col((MediaOfferIden::Table, MediaOfferIden::OfferId))
                        .eq(offer_id),
                );
                Ok(())
            }
        }
    }

    fn add_order_by(
        query: &mut SelectStatement,
        order_by_field: MediaOrderByField,
        order_by_direction: Direction,
    ) {
        use MediaOrderByField::*;

        let order = match order_by_direction {
            Direction::Unspecified | Direction::Asc => Order::Asc,
            Direction::Desc => Order::Desc,
        };

        match order_by_field {
            Unspecified | CreatedAt => {
                query.order_by((MediaIden::Table, MediaIden::CreatedAt), order);
            }
            UpdatedAt => {
                query.order_by((MediaIden::Table, MediaIden::UpdatedAt), order);
            }
            Ordering => {
                query.order_by(
                    (MediaOfferIden::Table, MediaOfferIden::Ordering),
                    order,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create<'a>(
        transaction: &Transaction<'a>,
        media_id: &Uuid,
        shop_id: &Uuid,
        user_id: &String,
        name: &String,
        file_path: &String,
        size_bytes: i64,
        file_name: &String,
    ) -> Result<Self, DbError> {
        let (sql, values) = Query::insert()
            .into_table(MediaIden::Table)
            .columns([
                MediaIden::MediaId,
                MediaIden::ShopId,
                MediaIden::UserId,
                MediaIden::Name,
                MediaIden::DataUrl,
                MediaIden::SizeBytes,
                MediaIden::FileName,
            ])
            .values([
                (*media_id).into(),
                (*shop_id).into(),
                user_id.into(),
                name.into(),
                file_path.into(),
                size_bytes.into(),
                file_name.into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = transaction
            .query_one(sql.as_str(), &values.as_params())
            .await?;

        Ok(Self::from(row))
    }

    pub async fn get_for_owner(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(MediaIden::Table)
            .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }

    pub async fn get_accessible(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Self::select_accessible(user_id)
            .column((MediaIden::Table, Asterisk))
            .and_where(
                Expr::col((MediaIden::Table, MediaIden::MediaId)).eq(*media_id),
            )
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }

    pub async fn list(
        pool: &Pool,
        shop_id: &Uuid,
        user_id: &String,
        limit: u64,
        offset: u64,
        filter: Option<(MediaFilterField, String)>,
        order_by: Option<(MediaOrderByField, Direction)>,
    ) -> Result<(Vec<Self>, i64), DbError> {
        let conn = pool.get().await?;

        let ((sql, values), (count_sql, count_values)) = {
            let mut query = Self::select_with_offer_ids();
            let mut count_query = Self::select_count();

            query
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::ShopId))
                        .eq(*shop_id),
                )
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::UserId))
                        .eq(user_id),
                );

            count_query
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::ShopId))
                        .eq(*shop_id),
                )
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::UserId))
                        .eq(user_id),
                );

            if let Some((filter_field, filter_query)) = filter {
                Self::add_filter(
                    &mut query,
                    filter_field,
                    filter_query.clone(),
                )?;
                Self::add_filter(&mut count_query, filter_field, filter_query)?;
            }

            if let Some((order_by_field, order_by_direction)) = order_by {
                Self::add_order_by(
                    &mut query,
                    order_by_field,
                    order_by_direction,
                );
            }

            (
                query
                    .column((MediaIden::Table, Asterisk))
                    .limit(limit)
                    .offset(offset)
                    .build_postgres(PostgresQueryBuilder),
                count_query
                    .expr(Expr::col((MediaIden::Table, Asterisk)).count())
                    .build_postgres(PostgresQueryBuilder),
            )
        };

        let rows = conn.query(sql.as_str(), &values.as_params()).await?;
        let count_rows = conn
            .query(count_sql.as_str(), &count_values.as_params())
            .await?;

        let count = get_count_from_rows(&count_rows);

        Ok((rows.iter().map(Self::from).collect(), count))
    }

    pub async fn list_all_for_user(
        pool: &Pool,
        user_id: &String,
    ) -> Result<Vec<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(MediaIden::Table)
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        let rows = client.query(sql.as_str(), &values.as_params()).await?;

        Ok(rows.iter().map(Self::from).collect())
    }

    pub async fn list_accessible(
        pool: &Pool,
        user_id: &String,
        limit: u64,
        offset: u64,
        filter: Option<(MediaFilterField, String)>,
        order_by: Option<(MediaOrderByField, Direction)>,
    ) -> Result<(Vec<Self>, i64), DbError> {
        let mut conn = pool.get().await?;
        let transaction = conn.transaction().await?;

        let ((sql, values), (count_sql, count_values)) = {
            let mut query = Self::select_accessible(user_id);

            if let Some((filter_field, filter_query)) = filter {
                Self::add_filter(&mut query, filter_field, filter_query)?;
            }

            let mut count_query = query.clone();

            if let Some((order_by_field, order_by_direction)) = order_by {
                Self::add_order_by(
                    &mut query,
                    order_by_field,
                    order_by_direction,
                );
            }

            (
                query
                    .column((MediaIden::Table, Asterisk))
                    .limit(limit)
                    .offset(offset)
                    .build_postgres(PostgresQueryBuilder),
                count_query
                    .expr(Expr::col((MediaIden::Table, Asterisk)).count())
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

    pub async fn update(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
        name: Option<String>,
        size_bytes: Option<i64>,
        file_name: Option<String>,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::update();
            query.table(MediaIden::Table);

            if let Some(name) = name {
                query.value(MediaIden::Name, name);
            }

            if let Some(size_bytes) = size_bytes {
                query.value(MediaIden::SizeBytes, size_bytes);
            }

            if let Some(file_name) = file_name {
                query.value(MediaIden::FileName, file_name);
            }

            query
                .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
                .and_where(Expr::col(MediaIden::UserId).eq(user_id))
                .returning_all()
                .build_postgres(PostgresQueryBuilder)
        };

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn add_size(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
        additional_size: i64,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::update()
            .table(MediaIden::Table)
            .value(
                MediaIden::SizeBytes,
                Expr::cust_with_values("size_bytes + $1", [additional_size]),
            )
            .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn delete(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
    ) -> Result<(), DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(MediaIden::Table)
            .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        conn.execute(sql.as_str(), &values.as_params()).await?;

        Ok(())
    }

    pub async fn begin_delete<'a>(
        transaction: &Transaction<'a>,
        media_id: &Uuid,
        user_id: &String,
    ) -> Result<(), DbError> {
        let (sql, values) = Query::delete()
            .from_table(MediaIden::Table)
            .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        transaction
            .execute(sql.as_str(), &values.as_params())
            .await?;

        Ok(())
    }
}

impl From<&Row> for Media {
    fn from(row: &Row) -> Self {
        let media_offers: Option<MediaOffersVec> =
            row.try_get(Self::MEDIA_OFFERS_ALIAS).ok();

        Self {
            media_id: row.get(MediaIden::MediaId.to_string().as_str()),
            offer_ids: media_offers
                .clone()
                .map(|mo| mo.0.into_iter().map(|m| m.offer_id).collect()),
            shop_id: row.get(MediaIden::ShopId.to_string().as_str()),
            user_id: row.get(MediaIden::UserId.to_string().as_str()),
            created_at: row.get(MediaIden::CreatedAt.to_string().as_str()),
            updated_at: row.get(MediaIden::UpdatedAt.to_string().as_str()),
            name: row.get(MediaIden::Name.to_string().as_str()),
            data_url: row.get(MediaIden::DataUrl.to_string().as_str()),
            size_bytes: u64::try_from(
                row.get::<&str, i64>(MediaIden::SizeBytes.to_string().as_str()),
            )
            .expect("should fit"),
            file_name: row.get(MediaIden::FileName.to_string().as_str()),
            ordering: media_offers
                .and_then(|mo| mo.0.first().map(|m| m.ordering))
                .unwrap_or(0),
        }
    }
}

impl From<Row> for Media {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
