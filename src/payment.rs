use tonic::transport::Channel;
use tonic::{Request, Status};
use uuid::Uuid;

use crate::api::peoplesmarkets::payment::v1::stripe_service_client::StripeServiceClient;
use crate::api::peoplesmarkets::payment::v1::{
    CancelSubscriptionRequest, ResumeSubscriptionRequest,
};
use crate::CredentialsService;

pub struct PaymentService {
    stripe_service_client: StripeServiceClient<Channel>,
    credentials_service: CredentialsService,
}

impl PaymentService {
    pub async fn init(
        url: String,
        credentials_service: CredentialsService,
    ) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            stripe_service_client: StripeServiceClient::connect(url).await?,
            credentials_service,
        })
    }

    pub async fn cancel_stripe_subscription(
        &self,
        shop_id: &Uuid,
        stripe_subscription_id: String,
    ) -> Result<(), Status> {
        let mut client = self.stripe_service_client.clone();

        let mut request = Request::new(CancelSubscriptionRequest {
            shop_id: shop_id.to_string(),
            stripe_subscription_id,
        });

        self.credentials_service
            .with_auth_header(&mut request)
            .await?;

        client.cancel_subscription(request).await?;

        Ok(())
    }

    pub async fn resume_stripe_subscription(
        &self,
        shop_id: &Uuid,
        stripe_subscription_id: String,
    ) -> Result<(), Status> {
        let mut client = self.stripe_service_client.clone();

        let mut request = Request::new(ResumeSubscriptionRequest {
            shop_id: shop_id.to_string(),
            stripe_subscription_id,
        });

        self.credentials_service
            .with_auth_header(&mut request)
            .await?;

        client.resume_subscription(request).await?;

        Ok(())
    }
}
