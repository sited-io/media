use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{
    all, Asterisk, Expr, Iden, OnConflict, PostgresQueryBuilder, Query,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::DbError;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "sub_offers")]
enum SubOfferIden {
    Table,
    OfferId,
    ShopId,
    UserId,
}

pub struct SubOffer {
    pub offer_id: Uuid,
    pub shop_id: Uuid,
    pub user_id: String,
}

impl SubOffer {
    pub async fn upsert(
        pool: &Pool,
        offer_id: &Uuid,
        shop_id: &Uuid,
        user_id: &String,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubOfferIden::Table)
            .columns([
                SubOfferIden::OfferId,
                SubOfferIden::ShopId,
                SubOfferIden::UserId,
            ])
            .values([(*offer_id).into(), (*shop_id).into(), user_id.into()])?
            .on_conflict(
                OnConflict::column(SubOfferIden::OfferId)
                    .update_columns([
                        SubOfferIden::ShopId,
                        SubOfferIden::UserId,
                    ])
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_one(sql.as_str(), &values.as_params())
            .await?
            .into())
    }

    pub async fn get_for_owner(
        pool: &Pool,
        offer_id: &Uuid,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(SubOfferIden::Table)
            .cond_where(all![
                Expr::col(SubOfferIden::OfferId).eq(*offer_id),
                Expr::col(SubOfferIden::UserId).eq(user_id)
            ])
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_opt(sql.as_str(), &values.as_params())
            .await?
            .map(Self::from))
    }

    pub async fn delete(pool: &Pool, offer_id: &Uuid) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(SubOfferIden::Table)
            .and_where(Expr::col(SubOfferIden::OfferId).eq(*offer_id))
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_one(sql.as_str(), &values.as_params())
            .await?
            .into())
    }
}

impl From<Row> for SubOffer {
    fn from(row: Row) -> Self {
        Self {
            offer_id: row.get(SubOfferIden::OfferId.to_string().as_str()),
            shop_id: row.get(SubOfferIden::ShopId.to_string().as_str()),
            user_id: row.get(SubOfferIden::UserId.to_string().as_str()),
        }
    }
}
