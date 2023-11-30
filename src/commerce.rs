use http::header::AUTHORIZATION;
use tonic::metadata::MetadataMap;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::api::peoplesmarkets::commerce::v1::offer_service_client::OfferServiceClient;
use crate::api::peoplesmarkets::commerce::v1::shop_service_client::ShopServiceClient;
use crate::api::peoplesmarkets::commerce::v1::{
    GetOfferRequest, GetShopRequest,
};

pub struct CommerceService {
    shop_client: ShopServiceClient<Channel>,
    offer_client: OfferServiceClient<Channel>,
}

impl CommerceService {
    pub async fn init(url: String) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            shop_client: ShopServiceClient::connect(url.clone()).await?,
            offer_client: OfferServiceClient::connect(url).await?,
        })
    }

    pub async fn check_shop_and_owner(
        &self,
        shop_id: &String,
        user_id: &String,
        metadata: &MetadataMap,
    ) -> Result<(), Status> {
        let mut client = self.shop_client.clone();

        let mut request = Request::new(GetShopRequest {
            shop_id: Some(shop_id.to_owned()),
            extended: None,
            ..Default::default()
        });

        if let Some(auth_header) = metadata.get(AUTHORIZATION.as_str()) {
            request
                .metadata_mut()
                .insert(AUTHORIZATION.as_str(), auth_header.to_owned());
        }

        let shop = client
            .get_shop(request)
            .await
            .map_err(|err| {
                tracing::error!("{}", err);
                Status::not_found("shop")
            })?
            .into_inner()
            .shop
            .ok_or_else(|| Status::not_found("shop response was empty"))?;

        if shop.user_id == *user_id {
            Ok(())
        } else {
            Err(Status::not_found("user is not owner of this shop"))
        }
    }

    pub async fn check_offer_and_owner(
        &self,
        offer_id: &String,
        user_id: &String,
        metadata: &MetadataMap,
    ) -> Result<(), Status> {
        let mut client = self.offer_client.clone();

        let mut request = Request::new(GetOfferRequest {
            offer_id: offer_id.to_owned(),
        });

        if let Some(token) = metadata.get(AUTHORIZATION.as_str()) {
            request
                .metadata_mut()
                .insert(AUTHORIZATION.as_str(), token.to_owned());
        }

        let offer = client
            .get_offer(request)
            .await
            .map_err(|err| {
                tracing::error!("{}", err);
                Status::not_found("offer")
            })?
            .into_inner()
            .offer
            .ok_or_else(|| Status::not_found("offer response was empty"))?;

        if offer.user_id == *user_id {
            Ok(())
        } else {
            Err(Status::not_found("user is not owner of this offer"))
        }
    }
}
