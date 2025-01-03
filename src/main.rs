use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderName, Method};
use tonic::transport::Server;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use media::api::sited_io::media::v1::media_service_server::MediaServiceServer;
use media::db::{init_db_pool, migrate};
use media::files::FileService;
use media::logging::{LogOnFailure, LogOnRequest, LogOnResponse};
use media::subscribers::{
    OfferSubscriber, ShopSubscriber, SubscriptionSubscriber,
};
use media::{
    get_env_var, init_jwks_verifier, CredentialsService, MediaService,
    MediaSubscriptionService, PaymentService, QuotaService,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging
    tracing_subscriber::fmt::init();

    // get required environment variables
    let host = get_env_var("HOST");

    let jwks_url = get_env_var("JWKS_URL");
    let jwks_host = get_env_var("JWKS_HOST");

    // initialize database connection and migrate
    let db_pool = init_db_pool(
        get_env_var("DB_HOST"),
        get_env_var("DB_PORT").parse().unwrap(),
        get_env_var("DB_USER"),
        get_env_var("DB_PASSWORD"),
        get_env_var("DB_DBNAME"),
        std::env::var("DB_ROOT_CERT").ok(),
    )?;
    // migrate(&db_pool).await?;

    // initialize credentials service
    let credentials_service = CredentialsService::new(
        get_env_var("OAUTH_URL"),
        get_env_var("OAUTH_HOST"),
        get_env_var("SERVICE_USER_CLIENT_ID"),
        get_env_var("SERVICE_USER_CLIENT_SECRET"),
    );

    // initialize file service
    let file_service = FileService::new(
        get_env_var("BUCKET_NAME"),
        get_env_var("BUCKET_ENDPOINT"),
        get_env_var("BUCKET_ACCESS_KEY_ID"),
        get_env_var("BUCKET_SECRET_ACCESS_KEY"),
    )
    .await;

    // initialize payment service
    let payment_service = PaymentService::init(
        get_env_var("PAYMENT_SERVICE_URL"),
        credentials_service,
    )
    .await?;

    // initialize quota service
    let quota_service = QuotaService::new(
        db_pool.clone(),
        get_env_var("DEFAULT_USER_QUOTA_MIB").parse().unwrap(),
    );

    // initialize NATS client
    let nats_client = async_nats::ConnectOptions::new()
        .user_and_password(
            get_env_var("NATS_USER"),
            get_env_var("NATS_PASSWORD"),
        )
        .connect(get_env_var("NATS_HOST"))
        .await?;

    // initialize subscribers
    let shop_subscriber =
        ShopSubscriber::new(nats_client.clone(), db_pool.clone());
    let offer_subscriber =
        OfferSubscriber::new(nats_client.clone(), db_pool.clone());
    let subscription_subscriber =
        SubscriptionSubscriber::new(nats_client.clone(), db_pool.clone());

    let media_service = MediaService::build(
        db_pool.clone(),
        init_jwks_verifier(&jwks_host, &jwks_url)?,
        file_service,
        quota_service,
        get_env_var("MAX_MESSAGE_SIZE_BYTES").parse().unwrap(),
    );

    let media_subscription_service = MediaSubscriptionService::build(
        db_pool,
        init_jwks_verifier(&jwks_host, &jwks_url)?,
        payment_service,
    );

    // configure gRPC health reporter
    let (mut health_reporter, health_service) =
        tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<MediaServiceServer<MediaService>>()
        .await;

    // configure gRPC reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            tonic_health::pb::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            media::api::sited_io::FILE_DESCRIPTOR_SET,
        )
        .build_v1()
        .unwrap();

    tracing::log::info!("gRPC+web server listening on {}", host);

    let shop_subscriber_handle =
        tokio::spawn(async move { shop_subscriber.subscribe().await });

    let offer_subscriber_handle =
        tokio::spawn(async move { offer_subscriber.subscribe().await });

    let subscription_subscriber_handle =
        tokio::spawn(async move { subscription_subscriber.subscribe().await });

    let server_handle = tokio::spawn(async move {
        Server::builder()
            .layer(
                TraceLayer::new_for_grpc()
                    .on_request(LogOnRequest::default())
                    .on_response(LogOnResponse::default())
                    .on_failure(LogOnFailure::default()),
            )
            .layer(
                CorsLayer::new()
                    .allow_headers([
                        AUTHORIZATION,
                        ACCEPT,
                        CONTENT_TYPE,
                        HeaderName::from_static("grpc-status"),
                        HeaderName::from_static("grpc-message"),
                        HeaderName::from_static("x-grpc-web"),
                        HeaderName::from_static("x-user-agent"),
                    ])
                    .allow_methods([Method::POST])
                    .allow_origin(AllowOrigin::any())
                    .allow_private_network(true),
            )
            .accept_http1(true)
            .add_service(tonic_web::enable(reflection_service))
            .add_service(tonic_web::enable(health_service))
            .add_service(tonic_web::enable(media_service))
            .add_service(tonic_web::enable(media_subscription_service))
            .serve(host.parse().unwrap())
            .await
    });

    tokio::join!(
        server_handle,
        shop_subscriber_handle,
        offer_subscriber_handle,
        subscription_subscriber_handle,
    )
    .0??;

    Ok(())
}
