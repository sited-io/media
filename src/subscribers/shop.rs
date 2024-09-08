use deadpool_postgres::Pool;
use futures::StreamExt;
use prost::Message;

use crate::api::sited_io::commerce::v1::ShopResponse;
use crate::model::SubShop;

pub struct ShopSubscriber {
    client: async_nats::Client,
    pool: Pool,
}

impl ShopSubscriber {
    pub fn new(client: async_nats::Client, pool: Pool) -> Self {
        Self { client, pool }
    }

    pub async fn subscribe(&self) {
        let mut subscriber = self
            .client
            .queue_subscribe("commerce.shop.>", "commerce.shop".to_string())
            .await
            .unwrap();

        while let Some(message) = subscriber.next().await {
            let action: &str =
                message.subject.split('.').last().unwrap_or_default();

            let Ok(shop_response) = ShopResponse::decode(message.payload)
            else {
                tracing::error!("[ShopSubscriber.subscribe]: could not decode message for subject {}", message.subject);
                return;
            };

            let Ok(shop_id) = shop_response.shop_id.parse() else {
                tracing::error!("[ShopSubscriber.subscribe]: could not parse shop_id as UUID");
                return;
            };

            if let Err(err) = match action {
                "upsert" => {
                    SubShop::upsert(
                        &self.pool,
                        &shop_id,
                        &shop_response.user_id,
                    )
                    .await
                }
                "delete" => SubShop::delete(&self.pool, &shop_id).await,
                unexpected => {
                    tracing::error!("[ShopSubscriber.subscribe]: Unexpected action: '{unexpected}'");
                    return;
                }
            } {
                tracing::error!("[ShopSubscriber.subscribe]: {:?}", err);
            }
        }
    }
}
