use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use futures::StreamExt;
use prost::Message;

use crate::api::sited_io::media::v1::MediaSubscriptionResponse;
use crate::model::MediaSubscription;

pub struct SubscriptionSubscriber {
    client: async_nats::Client,
    pool: Pool,
}

fn ts_to_dt(timestamp: u64) -> Option<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(timestamp.try_into().unwrap(), 0)
}

fn from_response(
    response: &MediaSubscriptionResponse,
) -> Option<MediaSubscription> {
    Some(MediaSubscription {
        media_subscription_id: response.media_subscription_id.parse().ok()?,
        buyer_user_id: response.buyer_user_id.clone(),
        offer_id: response.offer_id.parse().ok()?,
        shop_id: response.shop_id.parse().ok()?,
        current_period_start: ts_to_dt(response.current_period_start)?,
        current_period_end: ts_to_dt(response.current_period_end)?,
        subscription_status: response.subscription_status.clone(),
        payed_at: ts_to_dt(response.payed_at)?,
        payed_until: ts_to_dt(response.payed_until)?,
        created_at: Default::default(),
        updated_at: Default::default(),
        stripe_subscription_id: response.stripe_subscription_id.clone(),
        canceled_at: response.canceled_at.and_then(ts_to_dt),
        cancel_at: response.canceled_at.and_then(ts_to_dt),
    })
}

impl SubscriptionSubscriber {
    pub fn new(client: async_nats::Client, pool: Pool) -> Self {
        Self { client, pool }
    }

    pub async fn subscribe(&self) {
        let mut subscriber = self
            .client
            .queue_subscribe(
                "stripe-webhooks.subscription.>",
                "media.subscription".to_string(),
            )
            .await
            .unwrap();

        while let Some(message) = subscriber.next().await {
            let action: &str =
                message.subject.split('.').last().unwrap_or_default();

            let Ok(media_subscription_response) =
                MediaSubscriptionResponse::decode(message.payload)
            else {
                tracing::error!(
                    "[SubscriptionSubscriber.subscribe]: could not decode message for subject {}",
                    message.subject,
                );
                continue;
            };

            let Some(media_subscription) =
                from_response(&media_subscription_response)
            else {
                tracing::error!(
                    "[SubscriptionSubscriber.subscribe]: could not convert message to MediaSubscription",
                );
                continue;
            };

            if let Err(err) = match action {
                "upsert" => {
                    MediaSubscription::upsert(&self.pool, media_subscription)
                        .await
                }
                "delete" => {
                    MediaSubscription::delete(
                        &self.pool,
                        &media_subscription.media_subscription_id,
                    )
                    .await
                }
                unexpected => {
                    tracing::error!(
                        "[SubscriptionSubscriber.subscribe]: Unexpected action: '{}'",
                        unexpected,
                    );
                    continue;
                }
            } {
                tracing::error!(
                    "[SubscriptionSubscriber.subscribe]: {:?}",
                    err
                );
                continue;
            }

            tracing::info!(
                "[SubscriptionSubscriber.subscribe]: action {} on subscription {} successful",
                action,
                &media_subscription_response.media_subscription_id,
            );
        }
    }
}
