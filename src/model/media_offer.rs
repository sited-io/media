use sea_query::{Func, Iden, IntoColumnRef, SimpleExpr};
use uuid::Uuid;

use crate::db::ArrayAgg;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "medias_offers")]
pub enum MediaOfferIden {
    Table,
    MediaId,
    OfferId,
}

pub struct MediaOfferAsRel {
    pub media_id: Uuid,
    pub offer_id: Uuid,
}

impl MediaOfferAsRel {
    pub fn get_agg() -> SimpleExpr {
        Func::cust(ArrayAgg)
            .arg(SimpleExpr::Column(
                (MediaOfferIden::Table, MediaOfferIden::OfferId)
                    .into_column_ref(),
            ))
            .into()
    }
}
