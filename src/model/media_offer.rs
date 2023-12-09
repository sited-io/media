use deadpool_postgres::tokio_postgres::types::{private, FromSql, Type};
use deadpool_postgres::Pool;
use fallible_iterator::FallibleIterator;
use postgres_protocol::types;
use sea_query::{
    Asterisk, Expr, Func, Iden, PostgresQueryBuilder, Query, SimpleExpr,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::{get_type_from_oid, ArrayAgg, DbError};

#[derive(Debug, Clone, Iden)]
#[iden(rename = "medias_offers")]
pub enum MediaOfferIden {
    Table,
    MediaId,
    OfferId,
    UserId,
    Ordering,
}

#[derive(Debug, Clone)]
pub struct MediaOffer {
    pub media_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: String,
    pub ordering: i64,
}

impl MediaOffer {
    pub fn get_agg() -> SimpleExpr {
        Func::cust(ArrayAgg)
            .args([Expr::tuple([
                Expr::col((MediaOfferIden::Table, MediaOfferIden::MediaId))
                    .into(),
                Expr::col((MediaOfferIden::Table, MediaOfferIden::OfferId))
                    .into(),
                Expr::col((MediaOfferIden::Table, MediaOfferIden::UserId))
                    .into(),
                Expr::col((MediaOfferIden::Table, MediaOfferIden::Ordering))
                    .into(),
            ])
            .into()])
            .into()
    }

    pub async fn create(
        pool: &Pool,
        media_id: &Uuid,
        offer_id: &Uuid,
        user_id: &String,
        ordering: i64,
    ) -> Result<(), DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaOfferIden::Table)
            .columns([
                MediaOfferIden::MediaId,
                MediaOfferIden::OfferId,
                MediaOfferIden::UserId,
                MediaOfferIden::Ordering,
            ])
            .values([
                (*media_id).into(),
                (*offer_id).into(),
                user_id.into(),
                ordering.into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        client.execute(sql.as_str(), &values.as_params()).await?;

        Ok(())
    }

    pub async fn get_highest_ordering(
        pool: &Pool,
        offer_id: &Uuid,
        user_id: &String,
    ) -> Result<i64, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(MediaOfferIden::Table)
            .and_where(Expr::col(MediaOfferIden::OfferId).eq(*offer_id))
            .and_where(Expr::col(MediaOfferIden::UserId).eq(user_id))
            .order_by(MediaOfferIden::Ordering, sea_query::Order::Desc)
            .limit(1)
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_opt(sql.as_str(), &values.as_params()).await?;

        let ordering: i64 = if let Some(row) = row {
            row.get(MediaOfferIden::Ordering.to_string().as_str())
        } else {
            0
        };

        Ok(ordering)
    }

    pub async fn update_ordering(
        pool: &Pool,
        media_id: &Uuid,
        offer_id: &Uuid,
        user_id: &String,
        ordering: i64,
    ) -> Result<(), DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::update()
            .table(MediaOfferIden::Table)
            .value(MediaOfferIden::Ordering, ordering)
            .and_where(Expr::col(MediaOfferIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaOfferIden::OfferId).eq(*offer_id))
            .and_where(Expr::col(MediaOfferIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        conn.execute(sql.as_str(), &values.as_params()).await?;

        Ok(())
    }

    pub async fn delete(
        pool: &Pool,
        media_id: &Uuid,
        offer_id: &Uuid,
        user_id: &String,
    ) -> Result<(), DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(MediaOfferIden::Table)
            .and_where(Expr::col(MediaOfferIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaOfferIden::OfferId).eq(*offer_id))
            .and_where(Expr::col(MediaOfferIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        client.execute(sql.as_str(), &values.as_params()).await?;

        Ok(())
    }
}

impl<'a> FromSql<'a> for MediaOffer {
    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::RECORD => true,
            _ => {
                tracing::log::error!("OfferImageAsRel FromSql accepts: postgres type {:?} not implemented", ty);
                false
            }
        }
    }

    fn from_sql(
        _: &deadpool_postgres::tokio_postgres::types::Type,
        mut raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        private::read_be_i32(&mut raw)?;

        let oid = private::read_be_i32(&mut raw)?;
        let ty = get_type_from_oid::<Uuid>(oid)?;
        let media_id: Uuid = private::read_value(&ty, &mut raw)?;

        let oid = private::read_be_i32(&mut raw)?;
        let ty = get_type_from_oid::<Uuid>(oid)?;
        let offer_id: Uuid = private::read_value(&ty, &mut raw)?;

        let oid = private::read_be_i32(&mut raw)?;
        let ty = get_type_from_oid::<String>(oid)?;
        let user_id: String = private::read_value(&ty, &mut raw)?;

        let oid = private::read_be_i32(&mut raw)?;
        let ty = get_type_from_oid::<i64>(oid)?;
        let ordering: i64 = private::read_value(&ty, &mut raw)?;

        Ok(Self {
            media_id,
            offer_id,
            user_id,
            ordering,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MediaOffersVec(pub Vec<MediaOffer>);

impl<'a> FromSql<'a> for MediaOffersVec {
    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::RECORD_ARRAY => true,
            _ => {
                tracing::log::error!("OfferImageAsRelVec FromSql accepts: postgres type {:?} not implemented", ty);
                false
            }
        }
    }

    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let array = types::array_from_sql(raw)?;

        if array.dimensions().count()? > 1 {
            return Err("array contains too many dimensions".into());
        }

        Ok(Self(
            array
                .values()
                .filter_map(|v| {
                    Ok(MediaOffer::from_sql_nullable(&Type::RECORD, v).ok())
                })
                .collect()?,
        ))
    }
}
