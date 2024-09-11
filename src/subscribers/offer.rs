use deadpool_postgres::Pool;
use futures::StreamExt;
use prost::Message;

use crate::api::sited_io::commerce::v1::OfferResponse;
use crate::model::SubOffer;

pub struct OfferSubscriber {
    client: async_nats::Client,
    pool: Pool,
}

impl OfferSubscriber {
    pub fn new(client: async_nats::Client, pool: Pool) -> Self {
        Self { client, pool }
    }

    pub async fn subscribe(&self) {
        let mut subscriber = self
            .client
            .queue_subscribe("commerce.offer.>", "media.offer".to_string())
            .await
            .unwrap();

        while let Some(message) = subscriber.next().await {
            let action: &str =
                message.subject.split('.').last().unwrap_or_default();

            let Ok(offer_response) = OfferResponse::decode(message.payload)
            else {
                tracing::error!("[OfferSubscriber.subscribe]: could not decode offer response");
                continue;
            };

            let Ok(offer_id) = offer_response.offer_id.parse() else {
                tracing::error!("[OfferSubscriber.subscribe]: could not parse offer_id as UUID");
                continue;
            };

            let Ok(shop_id) = offer_response.shop_id.parse() else {
                tracing::error!("[OfferSubscriber.subscribe]: could not parse shop_id as UUID");
                continue;
            };

            if let Err(err) = match action {
                "upsert" => {
                    SubOffer::upsert(
                        &self.pool,
                        &offer_id,
                        &shop_id,
                        &offer_response.user_id,
                    )
                    .await
                }
                "delete" => SubOffer::delete(&self.pool, &offer_id).await,
                unexpected => {
                    tracing::error!("[OfferSubscriber.subscribe]: Unexpected action: '{unexpected}'");
                    continue;
                }
            } {
                tracing::error!("[OfferSubscriber.subscribe]: {:?}", err);
            }
        }
    }
}
