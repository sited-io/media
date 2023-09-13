use http::header::AUTHORIZATION;
use jwtk::jwk::RemoteJwksVerifier;
use jwtk::Claims;
use tonic::metadata::MetadataMap;
use tonic::Status;

pub async fn get_user_id(
    metadata: &MetadataMap,
    verifier: &RemoteJwksVerifier,
) -> Result<String, Status> {
    let token = metadata
        .get(AUTHORIZATION.as_str())
        .and_then(|v| v.to_str().ok())
        .and_then(|header_value| header_value.split_once(' '))
        .map(|(_, token)| token.to_string())
        .ok_or_else(|| Status::unauthenticated(""))?;

    verifier
        .verify::<Claims<()>>(&token)
        .await
        .map_err(|err| Status::unauthenticated(err.to_string()))?
        .claims()
        .sub
        .clone()
        .ok_or_else(|| Status::unauthenticated(""))
}
