use std::io::Result;

fn main() -> Result<()> {
    const MEDIA_PROTOS: &[&str] = &[
        "service-apis/proto/sited_io/media/v1/media.proto",
        "service-apis/proto/sited_io/media/v1/media_subscription.proto",
    ];

    const CLIENT_PROTOS: &[&str] =
        &["service-apis/proto/sited_io/payment/v1/stripe.proto"];

    const SUBSCRIBER_PROTOS: &[&str] = &[
        "service-apis/proto/sited_io/commerce/v1/shop.proto",
        "service-apis/proto/sited_io/commerce/v1/offer.proto",
    ];

    const INCLUDES: &[&str] = &["service-apis/proto"];

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .build_client(false)
        .compile(SUBSCRIBER_PROTOS, INCLUDES)?;

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .build_client(true)
        .compile(CLIENT_PROTOS, INCLUDES)?;

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path("src/api/FILE_DESCRIPTOR_SET")
        .build_server(true)
        .build_client(false)
        .compile(MEDIA_PROTOS, INCLUDES)?;

    Ok(())
}
