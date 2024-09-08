use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderName, Method};
use tonic::transport::Server;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use media::api::sited_io::media::v1::media_service_server::MediaServiceServer;
use media::db::{init_db_pool, migrate};
use media::files::FileService;
use media::logging::{LogOnFailure, LogOnRequest, LogOnResponse};
use media::{
    get_env_var, init_jwks_verifier, CommerceService, CredentialsService,
    MediaService, MediaSubscriptionService, PaymentService, QuotaService,
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
    migrate(&db_pool).await?;

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

    let max_message_size_bytes =
        get_env_var("MAX_MESSAGE_SIZE_BYTES").parse().unwrap();

    // initialize commerce service client
    let commerce_service =
        CommerceService::init(get_env_var("COMMERCE_SERVICE_URL")).await?;

    // initialize quota service
    let quota_service = QuotaService::new(
        db_pool.clone(),
        get_env_var("DEFAULT_USER_QUOTA_MIB").parse().unwrap(),
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

    let media_service = MediaService::build(
        db_pool.clone(),
        init_jwks_verifier(&jwks_host, &jwks_url)?,
        file_service,
        commerce_service,
        quota_service,
        max_message_size_bytes,
    );

    let media_subscription_service = MediaSubscriptionService::build(
        db_pool,
        init_jwks_verifier(&jwks_host, &jwks_url)?,
        payment_service,
    );

    tracing::log::info!("gRPC+web server listening on {}", host);

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
        .await?;

    Ok(())
}
