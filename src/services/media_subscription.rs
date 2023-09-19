use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use jwtk::jwk::RemoteJwksVerifier;
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::media::v1::{PutMediaSubscriptionRequest, PutMediaSubscriptionResponse};
use crate::api::peoplesmarkets::media::v1::media_subscription_service_server::{MediaSubscriptionServiceServer, self};
use crate::auth::verify_service_user;
use crate::model::MediaSubscription;

use super::parse_uuid;

pub struct MediaSubscriptionService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
}

impl MediaSubscriptionService {
    fn new(pool: Pool, verifier: RemoteJwksVerifier) -> Self {
        Self { pool, verifier }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
    ) -> MediaSubscriptionServiceServer<Self> {
        MediaSubscriptionServiceServer::new(Self::new(pool, verifier))
    }

    fn timestamp_to_datetime(timestamp: u64) -> Result<DateTime<Utc>, Status> {
        if let Ok(timestamp) = i64::try_from(timestamp) {
            DateTime::<Utc>::from_timestamp(timestamp, 0)
                .ok_or_else(|| Status::invalid_argument(timestamp.to_string()))
        } else {
            Err(Status::invalid_argument(timestamp.to_string()))
        }
    }
}

#[async_trait]
impl media_subscription_service_server::MediaSubscriptionService
    for MediaSubscriptionService
{
    async fn put_media_subscription(
        &self,
        request: Request<PutMediaSubscriptionRequest>,
    ) -> Result<Response<PutMediaSubscriptionResponse>, Status> {
        verify_service_user(request.metadata(), &self.verifier).await?;

        let PutMediaSubscriptionRequest {
            media_subscription_id,
            buyer_user_id,
            offer_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
            payed_until,
        } = request.into_inner();

        MediaSubscription::put(
            &self.pool,
            &parse_uuid(&media_subscription_id, "media_subscription_id")?,
            &buyer_user_id,
            &parse_uuid(&offer_id, "offer_id")?,
            &Self::timestamp_to_datetime(current_period_start)?,
            &Self::timestamp_to_datetime(current_period_end)?,
            &subscription_status,
            &Self::timestamp_to_datetime(payed_at)?,
            &Self::timestamp_to_datetime(payed_until)?,
        )
        .await?;

        Ok(Response::new(PutMediaSubscriptionResponse {}))
    }
}
