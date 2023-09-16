use deadpool_postgres::Pool;
use sea_query::{
    Expr, Func, Iden, IntoColumnRef, PostgresQueryBuilder, Query, SimpleExpr,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::{ArrayAgg, DbError};

#[derive(Debug, Clone, Iden)]
#[iden(rename = "medias_offers")]
pub enum MediaOfferIden {
    Table,
    MediaId,
    OfferId,
    UserId,
}

pub struct MediaOffer {
    pub media_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: String,
}

impl MediaOffer {
    pub fn get_agg() -> SimpleExpr {
        Func::cust(ArrayAgg)
            .arg(SimpleExpr::Column(
                (MediaOfferIden::Table, MediaOfferIden::OfferId)
                    .into_column_ref(),
            ))
            .into()
    }

    pub async fn create(
        pool: &Pool,
        media_id: &Uuid,
        offer_id: &Uuid,
        user_id: &String,
    ) -> Result<(), DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaOfferIden::Table)
            .columns([
                MediaOfferIden::MediaId,
                MediaOfferIden::OfferId,
                MediaOfferIden::UserId,
            ])
            .values([(*media_id).into(), (*offer_id).into(), user_id.into()])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        client.execute(sql.as_str(), &values.as_params()).await?;

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
