use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use jwtk::jwk::RemoteJwksVerifier;
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::media::v1::{PutMediaSubscriptionRequest, PutMediaSubscriptionResponse, CancelMediaSubscriptionRequest, CancelMediaSubscriptionResponse, ResumeMediaSubscriptionRequest, ResumeMediaSubscriptionResponse, GetMediaSubscriptionRequest, GetMediaSubscriptionResponse, ListMediaSubscriptionsRequest, ListMediaSubscriptionsResponse, MediaSubscriptionResponse};
use crate::api::peoplesmarkets::media::v1::media_subscription_service_server::{MediaSubscriptionServiceServer, self};
use crate::auth::{verify_service_user, get_user_id};
use crate::model::MediaSubscription;
use crate::payment::PaymentService;

use super::{
    get_limit_offset_from_pagination, parse_optional_uuid, parse_uuid,
};

pub struct MediaSubscriptionService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    payment_service: PaymentService,
}

impl MediaSubscriptionService {
    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        payment_service: PaymentService,
    ) -> Self {
        Self {
            pool,
            verifier,
            payment_service,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        payment_service: PaymentService,
    ) -> MediaSubscriptionServiceServer<Self> {
        MediaSubscriptionServiceServer::new(Self::new(
            pool,
            verifier,
            payment_service,
        ))
    }

    fn to_response(
        &self,
        media_subscription: MediaSubscription,
    ) -> MediaSubscriptionResponse {
        MediaSubscriptionResponse {
            media_subscription_id: media_subscription
                .media_subscription_id
                .to_string(),
            buyer_user_id: media_subscription.buyer_user_id,
            shop_id: media_subscription.shop_id.to_string(),
            offer_id: media_subscription.offer_id.to_string(),
            current_period_start: u64::try_from(
                media_subscription.current_period_start.timestamp(),
            )
            .unwrap(),
            current_period_end: u64::try_from(
                media_subscription.current_period_end.timestamp(),
            )
            .unwrap(),
            subscription_status: media_subscription.subscription_status,
            payed_at: u64::try_from(media_subscription.payed_at.timestamp())
                .unwrap(),
            payed_until: u64::try_from(
                media_subscription.payed_until.timestamp(),
            )
            .unwrap(),
            stripe_subscription_id: media_subscription.stripe_subscription_id,
            canceled_at: media_subscription
                .canceled_at
                .map(|c| u64::try_from(c.timestamp()).unwrap()),
            cancel_at: media_subscription
                .cancel_at
                .map(|c| u64::try_from(c.timestamp()).unwrap()),
        }
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
            shop_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
            payed_until,
            stripe_subscription_id,
            canceled_at,
            cancel_at,
        } = request.into_inner();

        MediaSubscription::put(
            &self.pool,
            &parse_uuid(&media_subscription_id, "media_subscription_id")?,
            &buyer_user_id,
            &parse_uuid(&offer_id, "offer_id")?,
            &parse_uuid(&shop_id, "shop_id")?,
            &Self::timestamp_to_datetime(current_period_start)?,
            &Self::timestamp_to_datetime(current_period_end)?,
            &subscription_status,
            &Self::timestamp_to_datetime(payed_at)?,
            &Self::timestamp_to_datetime(payed_until)?,
            stripe_subscription_id,
            canceled_at.and_then(|c| Self::timestamp_to_datetime(c).ok()),
            cancel_at.and_then(|c| Self::timestamp_to_datetime(c).ok()),
        )
        .await?;

        Ok(Response::new(PutMediaSubscriptionResponse {}))
    }

    async fn get_media_subscription(
        &self,
        request: Request<GetMediaSubscriptionRequest>,
    ) -> Result<Response<GetMediaSubscriptionResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let GetMediaSubscriptionRequest {
            media_subscription_id,
            offer_id,
        } = request.into_inner();

        let media_subscription_uuid = parse_optional_uuid(
            media_subscription_id,
            "media_subscription_id",
        )?;
        let offer_uuid = parse_optional_uuid(offer_id, "offer_id")?;

        let found_media_subscription = MediaSubscription::get(
            &self.pool,
            &user_id,
            media_subscription_uuid,
            offer_uuid,
        )
        .await?
        .ok_or(Status::not_found(""))?;

        Ok(Response::new(GetMediaSubscriptionResponse {
            media_subscription: Some(
                self.to_response(found_media_subscription),
            ),
        }))
    }

    async fn list_media_subscriptions(
        &self,
        request: Request<ListMediaSubscriptionsRequest>,
    ) -> Result<Response<ListMediaSubscriptionsResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let ListMediaSubscriptionsRequest {
            shop_id,
            pagination,
            is_accessible,
        } = request.into_inner();

        let shop_uuid = parse_optional_uuid(shop_id, "shop_id")?;

        let (limit, offset, mut pagination) =
            get_limit_offset_from_pagination(pagination)?;

        let (found_media_subscriptions, count) = MediaSubscription::list(
            &self.pool,
            &user_id,
            shop_uuid,
            is_accessible,
            limit.into(),
            offset.into(),
        )
        .await?;

        pagination.total_elements = count.try_into().map_err(|_| {
            Status::internal("Could not convert 'count' from i64 to u32")
        })?;

        Ok(Response::new(ListMediaSubscriptionsResponse {
            media_subscriptions: found_media_subscriptions
                .into_iter()
                .map(|f| self.to_response(f))
                .collect(),
            pagination: Some(pagination),
        }))
    }

    async fn cancel_media_subscription(
        &self,
        request: Request<CancelMediaSubscriptionRequest>,
    ) -> Result<Response<CancelMediaSubscriptionResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let CancelMediaSubscriptionRequest {
            media_subscription_id,
        } = request.into_inner();

        let media_subscription_uuid =
            parse_uuid(&media_subscription_id, "media_subscription_id")?;

        let found_media_subscritpion = MediaSubscription::get(
            &self.pool,
            &user_id,
            Some(media_subscription_uuid),
            None,
        )
        .await?
        .ok_or(Status::not_found(""))?;

        if let Some(stripe_subscription_id) =
            found_media_subscritpion.stripe_subscription_id
        {
            self.payment_service
                .cancel_stripe_subscription(
                    &found_media_subscritpion.shop_id,
                    stripe_subscription_id,
                )
                .await?;
        }

        Ok(Response::new(CancelMediaSubscriptionResponse {}))
    }

    async fn resume_media_subscription(
        &self,
        request: Request<ResumeMediaSubscriptionRequest>,
    ) -> Result<Response<ResumeMediaSubscriptionResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let ResumeMediaSubscriptionRequest {
            media_subscription_id,
        } = request.into_inner();

        let media_subscription_uuid =
            parse_uuid(&media_subscription_id, "media_subscription_id")?;

        let found_media_subscritpion = MediaSubscription::get(
            &self.pool,
            &user_id,
            Some(media_subscription_uuid),
            None,
        )
        .await?
        .ok_or(Status::not_found(format!(
            "media_subscription_id {}",
            media_subscription_id
        )))?;

        if let Some(stripe_subscription_id) =
            found_media_subscritpion.stripe_subscription_id
        {
            self.payment_service
                .resume_stripe_subscription(
                    &found_media_subscritpion.shop_id,
                    stripe_subscription_id,
                )
                .await?;
        }

        Ok(Response::new(ResumeMediaSubscriptionResponse {}))
    }
}
