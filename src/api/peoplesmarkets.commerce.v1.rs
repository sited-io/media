#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopCustomizationResponse {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    #[prost(uint64, tag = "4")]
    pub updated_at: u64,
    #[prost(string, optional, tag = "5")]
    pub logo_image_light_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub logo_image_dark_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub banner_image_light_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub banner_image_dark_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub show_banner_in_listing: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub show_banner_on_home: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub header_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "12")]
    pub header_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "13")]
    pub header_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "14")]
    pub header_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "15")]
    pub secondary_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "16")]
    pub secondary_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "17")]
    pub secondary_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "18")]
    pub secondary_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub header_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub header_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "4")]
    pub header_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "5")]
    pub header_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "6")]
    pub secondary_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "7")]
    pub secondary_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "8")]
    pub secondary_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "9")]
    pub secondary_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(message, optional, tag = "3")]
    pub image_dark: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(bool, optional, tag = "4")]
    pub show_in_listing: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub show_on_home: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(message, optional, tag = "3")]
    pub image_dark: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopResponse {}
/// Generated client implementations.
pub mod shop_customization_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShopCustomizationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShopCustomizationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ShopCustomizationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ShopCustomizationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ShopCustomizationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn put_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::PutShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShopCustomizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "PutShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopCustomizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/GetShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "GetShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopCustomizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/DeleteShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "DeleteShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_banner_image_to_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::PutBannerImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutBannerImageToShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutBannerImageToShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "PutBannerImageToShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_banner_image_from_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveBannerImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBannerImageFromShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/RemoveBannerImageFromShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "RemoveBannerImageFromShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_logo_image_to_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::PutLogoImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutLogoImageToShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutLogoImageToShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "PutLogoImageToShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_logo_image_from_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLogoImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLogoImageFromShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/RemoveLogoImageFromShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopCustomizationService",
                        "RemoveLogoImageFromShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopResponse {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    #[prost(uint64, tag = "4")]
    pub updated_at: u64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "7")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag = "9")]
    pub platform_fee_percent: u32,
    #[prost(uint32, tag = "10")]
    pub minimum_platform_fee_cent: u32,
    #[prost(message, optional, tag = "11")]
    pub customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShopRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "2")]
    pub extended: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopBySlugRequest {
    #[prost(string, tag = "1")]
    pub slug: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopBySlugResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopByDomainRequest {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopByDomainResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopsOrderBy {
    #[prost(enumeration = "ShopsOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopsFilter {
    #[prost(enumeration = "ShopsFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<ShopsOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<ShopsFilter>,
    #[prost(bool, optional, tag = "5")]
    pub extended: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsResponse {
    #[prost(message, repeated, tag = "1")]
    pub shops: ::prost::alloc::vec::Vec<ShopResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub slug: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShopsOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl ShopsOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShopsOrderByField::Unspecified => "SHOPS_ORDER_BY_FIELD_UNSPECIFIED",
            ShopsOrderByField::CreatedAt => "SHOPS_ORDER_BY_FIELD_CREATED_AT",
            ShopsOrderByField::UpdatedAt => "SHOPS_ORDER_BY_FIELD_UPDATED_AT",
            ShopsOrderByField::Name => "SHOPS_ORDER_BY_FIELD_NAME",
            ShopsOrderByField::Random => "SHOPS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOPS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOPS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "SHOPS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "SHOPS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "SHOPS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShopsFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
}
impl ShopsFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShopsFilterField::Unspecified => "SHOPS_FILTER_FIELD_UNSPECIFIED",
            ShopsFilterField::Name => "SHOPS_FILTER_FIELD_NAME",
            ShopsFilterField::Description => "SHOPS_FILTER_FIELD_DESCRIPTION",
            ShopsFilterField::NameAndDescription => {
                "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOPS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOPS_FILTER_FIELD_NAME" => Some(Self::Name),
            "SHOPS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Some(Self::NameAndDescription),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod shop_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShopServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShopServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ShopServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ShopServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ShopServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/CreateShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "CreateShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/GetShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("peoplesmarkets.commerce.v1.ShopService", "GetShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop_by_slug(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopBySlugRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopBySlugResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/GetShopBySlug",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "GetShopBySlug",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop_by_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopByDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopByDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/GetShopByDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "GetShopByDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_shops(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShopsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListShopsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/ListShops",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "ListShops",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/UpdateShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "UpdateShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.ShopService/DeleteShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.ShopService",
                        "DeleteShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recurring {
    #[prost(enumeration = "RecurringInterval", tag = "1")]
    pub interval: i32,
    #[prost(uint32, tag = "2")]
    pub interval_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    #[prost(enumeration = "Currency", tag = "1")]
    pub currency: i32,
    #[prost(enumeration = "PriceType", tag = "2")]
    pub price_type: i32,
    #[prost(enumeration = "PriceBillingScheme", tag = "3")]
    pub billing_scheme: i32,
    #[prost(uint32, tag = "4")]
    pub unit_amount: u32,
    #[prost(message, optional, tag = "5")]
    pub recurring: ::core::option::Option<Recurring>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    Unspecified = 0,
    Eur = 1,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Unspecified => "CURRENCY_UNSPECIFIED",
            Currency::Eur => "CURRENCY_EUR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CURRENCY_UNSPECIFIED" => Some(Self::Unspecified),
            "CURRENCY_EUR" => Some(Self::Eur),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    Unspecified = 0,
    OneTime = 1,
    Recurring = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::OneTime => "PRICE_TYPE_ONE_TIME",
            PriceType::Recurring => "PRICE_TYPE_RECURRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_ONE_TIME" => Some(Self::OneTime),
            "PRICE_TYPE_RECURRING" => Some(Self::Recurring),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceBillingScheme {
    Unspecified = 0,
    PerUnit = 1,
}
impl PriceBillingScheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceBillingScheme::Unspecified => "PRICE_BILLING_SCHEME_UNSPECIFIED",
            PriceBillingScheme::PerUnit => "PRICE_BILLING_SCHEME_PER_UNIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_BILLING_SCHEME_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_BILLING_SCHEME_PER_UNIT" => Some(Self::PerUnit),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecurringInterval {
    Unspecified = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
}
impl RecurringInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecurringInterval::Unspecified => "RECURRING_INTERVAL_UNSPECIFIED",
            RecurringInterval::Day => "RECURRING_INTERVAL_DAY",
            RecurringInterval::Week => "RECURRING_INTERVAL_WEEK",
            RecurringInterval::Month => "RECURRING_INTERVAL_MONTH",
            RecurringInterval::Year => "RECURRING_INTERVAL_YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECURRING_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "RECURRING_INTERVAL_DAY" => Some(Self::Day),
            "RECURRING_INTERVAL_WEEK" => Some(Self::Week),
            "RECURRING_INTERVAL_MONTH" => Some(Self::Month),
            "RECURRING_INTERVAL_YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferResponse {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shop_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shop_slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub shop_domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub created_at: i64,
    #[prost(int64, tag = "8")]
    pub updated_at: i64,
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub is_active: bool,
    #[prost(bool, tag = "12")]
    pub is_featured: bool,
    #[prost(enumeration = "OfferType", tag = "13")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "14")]
    pub images: ::prost::alloc::vec::Vec<OfferImageResponse>,
    #[prost(message, optional, tag = "15")]
    pub price: ::core::option::Option<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferImageResponse {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "OfferType", tag = "4")]
    pub r#type: i32,
    #[prost(bool, tag = "5")]
    pub is_featured: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersOrderBy {
    #[prost(enumeration = "OffersOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersFilter {
    #[prost(enumeration = "OffersFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub shop_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "4")]
    pub order_by: ::core::option::Option<OffersOrderBy>,
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<OffersFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersResponse {
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<OfferResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_active: ::core::option::Option<bool>,
    #[prost(enumeration = "OfferType", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub is_featured: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OfferType {
    Unspecified = 0,
    Physical = 1,
    Digital = 2,
    Service = 3,
}
impl OfferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OfferType::Unspecified => "OFFER_TYPE_UNSPECIFIED",
            OfferType::Physical => "OFFER_TYPE_PHYSICAL",
            OfferType::Digital => "OFFER_TYPE_DIGITAL",
            OfferType::Service => "OFFER_TYPE_SERVICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFER_TYPE_PHYSICAL" => Some(Self::Physical),
            "OFFER_TYPE_DIGITAL" => Some(Self::Digital),
            "OFFER_TYPE_SERVICE" => Some(Self::Service),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl OffersOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersOrderByField::Unspecified => "OFFERS_ORDER_BY_FIELD_UNSPECIFIED",
            OffersOrderByField::CreatedAt => "OFFERS_ORDER_BY_FIELD_CREATED_AT",
            OffersOrderByField::UpdatedAt => "OFFERS_ORDER_BY_FIELD_UPDATED_AT",
            OffersOrderByField::Name => "OFFERS_ORDER_BY_FIELD_NAME",
            OffersOrderByField::Random => "OFFERS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "OFFERS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "OFFERS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "OFFERS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
    Type = 4,
    IsFeatured = 5,
}
impl OffersFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersFilterField::Unspecified => "OFFERS_FILTER_FIELD_UNSPECIFIED",
            OffersFilterField::Name => "OFFERS_FILTER_FIELD_NAME",
            OffersFilterField::Description => "OFFERS_FILTER_FIELD_DESCRIPTION",
            OffersFilterField::NameAndDescription => {
                "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
            OffersFilterField::Type => "OFFERS_FILTER_FIELD_TYPE",
            OffersFilterField::IsFeatured => "OFFERS_FILTER_FIELD_IS_FEATURED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_FILTER_FIELD_NAME" => Some(Self::Name),
            "OFFERS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Some(Self::NameAndDescription),
            "OFFERS_FILTER_FIELD_TYPE" => Some(Self::Type),
            "OFFERS_FILTER_FIELD_IS_FEATURED" => Some(Self::IsFeatured),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod offer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OfferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OfferServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OfferServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OfferServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OfferServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/CreateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "CreateOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/GetOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "GetOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/ListOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "ListOffers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/UpdateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "UpdateOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/DeleteOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "DeleteOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_image_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddImageToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddImageToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/AddImageToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "AddImageToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_image_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/RemoveImageFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "RemoveImageFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_price_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::PutPriceToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutPriceToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/PutPriceToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "PutPriceToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_price_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePriceFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePriceFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.commerce.v1.OfferService/RemovePriceFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.OfferService",
                        "RemovePriceFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
