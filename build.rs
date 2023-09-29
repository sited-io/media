use std::io::Result;

fn main() -> Result<()> {
    const MEDIA_PROTOS: &[&str] = &[
        "service-apis/proto/peoplesmarkets/media/v1/media.proto",
        "service-apis/proto/peoplesmarkets/media/v1/media_subscription.proto",
    ];

    const COMMERCE_PROTOS: &[&str] = &[
        "service-apis/proto/peoplesmarkets/commerce/v1/shop.proto",
        "service-apis/proto/peoplesmarkets/commerce/v1/offer.proto",
    ];

    const INCLUDES: &[&str] = &["service-apis/proto"];

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .build_client(true)
        .compile(COMMERCE_PROTOS, INCLUDES)?;

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path("src/api/FILE_DESCRIPTOR_SET")
        .build_server(true)
        .build_client(false)
        .compile(MEDIA_PROTOS, INCLUDES)?;

    Ok(())
}
