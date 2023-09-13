use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::api::peoplesmarkets::commerce::v1::market_booth_service_client::MarketBoothServiceClient;
use crate::api::peoplesmarkets::commerce::v1::offer_service_client::OfferServiceClient;
use crate::api::peoplesmarkets::commerce::v1::GetMarketBoothRequest;

pub struct CommerceService {
    market_booth_client: MarketBoothServiceClient<Channel>,
    offer_client: OfferServiceClient<Channel>,
}

impl CommerceService {
    pub async fn init(url: String) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            market_booth_client: MarketBoothServiceClient::connect(url.clone())
                .await?,
            offer_client: OfferServiceClient::connect(url).await?,
        })
    }

    pub async fn check_market_booth_and_owner(
        &self,
        market_booth_id: &String,
        user_id: &String,
    ) -> Result<(), Status> {
        let mut client = self.market_booth_client.clone();

        let market_booth = client
            .get_market_booth(Request::new(GetMarketBoothRequest {
                market_booth_id: market_booth_id.to_owned(),
            }))
            .await
            .map_err(|_| Status::not_found("market_booth"))?
            .into_inner()
            .market_booth
            .ok_or_else(|| Status::not_found("market_booth"))?;

        if market_booth.user_id == *user_id {
            Ok(())
        } else {
            Err(Status::not_found("market_booth"))
        }
    }
}
