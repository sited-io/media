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
    ) -> Result<(), Status> {
        let mut client = self.shop_client.clone();

        let shop = client
            .get_shop(Request::new(GetShopRequest {
                shop_id: shop_id.to_owned(),
                extended: None,
            }))
            .await
            .map_err(|_| Status::not_found("shop"))?
            .into_inner()
            .shop
            .ok_or_else(|| Status::not_found("shop"))?;

        if shop.user_id == *user_id {
            Ok(())
        } else {
            Err(Status::not_found("shop"))
        }
    }

    pub async fn check_offer_and_owner(
        &self,
        offer_id: &String,
        user_id: &String,
    ) -> Result<(), Status> {
        let mut client = self.offer_client.clone();

        let offer = client
            .get_offer(Request::new(GetOfferRequest {
                offer_id: offer_id.to_owned(),
            }))
            .await
            .map_err(|_| Status::not_found("offer"))?
            .into_inner()
            .offer
            .ok_or_else(|| Status::not_found("offer"))?;

        if offer.user_id == *user_id {
            Ok(())
        } else {
            Err(Status::not_found("offer"))
        }
    }
}
