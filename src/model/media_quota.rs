use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{Asterisk, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;

use crate::db::DbError;

#[derive(Iden)]
#[iden(rename = "medias_quota")]
pub enum MediaQuotaIden {
    Table,
    UserId,
    MaxSizeMib,
}

#[derive(Debug, Clone)]
pub struct MediaQuota {
    pub user_id: String,
    pub max_size_mib: u64,
}

impl MediaQuota {
    pub async fn create(
        pool: &Pool,
        user_id: &String,
        max_size_mib: u64,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MediaQuotaIden::Table)
            .columns([MediaQuotaIden::UserId, MediaQuotaIden::MaxSizeMib])
            .values([
                user_id.into(),
                i64::try_from(max_size_mib).expect("should fit").into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn get(
        pool: &Pool,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(MediaQuotaIden::Table)
            .and_where(Expr::col(MediaQuotaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }
}

impl From<Row> for MediaQuota {
    fn from(row: Row) -> Self {
        Self {
            user_id: row.get(MediaQuotaIden::UserId.to_string().as_str()),
            max_size_mib: u64::try_from(row.get::<&str, i64>(
                MediaQuotaIden::MaxSizeMib.to_string().as_str(),
            ))
            .expect("Should not be negative and fit"),
        }
    }
}
