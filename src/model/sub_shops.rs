use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{
    all, Asterisk, Expr, Iden, OnConflict, PostgresQueryBuilder, Query,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::DbError;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "sub_shops")]
pub enum SubShopIden {
    Table,
    ShopId,
    UserId,
}

#[derive(Debug, Clone)]
pub struct SubShop {
    pub shop_id: Uuid,
    pub user_id: String,
}

impl SubShop {
    pub async fn upsert(
        pool: &Pool,
        shop_id: &Uuid,
        user_id: &String,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubShopIden::Table)
            .columns([SubShopIden::ShopId, SubShopIden::UserId])
            .values([(*shop_id).into(), user_id.into()])?
            .on_conflict(
                OnConflict::column(SubShopIden::ShopId)
                    .update_columns([SubShopIden::UserId])
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_one(sql.as_str(), &values.as_params())
            .await?
            .into())
    }

    pub async fn get_for_user(
        pool: &Pool,
        shop_id: &Uuid,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(SubShopIden::Table)
            .cond_where(all![
                Expr::col(SubShopIden::ShopId).eq(*shop_id),
                Expr::col(SubShopIden::UserId).eq(user_id)
            ])
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_opt(sql.as_str(), &values.as_params())
            .await?
            .map(Self::from))
    }

    pub async fn delete(pool: &Pool, shop_id: &Uuid) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(SubShopIden::Table)
            .and_where(Expr::col(SubShopIden::ShopId).eq(*shop_id))
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        Ok(conn
            .query_one(sql.as_str(), &values.as_params())
            .await?
            .into())
    }
}

impl From<Row> for SubShop {
    fn from(row: Row) -> Self {
        Self {
            shop_id: row.get(SubShopIden::ShopId.to_string().as_str()),
            user_id: row.get(SubShopIden::UserId.to_string().as_str()),
        }
    }
}
