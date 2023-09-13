use base64::Engine;
use deadpool_postgres::Pool;
use jwtk::jwk::RemoteJwksVerifier;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

use crate::api::peoplesmarkets::media::v1::media_service_server::{
    self, MediaServiceServer,
};
use crate::api::peoplesmarkets::media::v1::{
    CreateMediaRequest, CreateMediaResponse, DeleteMediaRequest,
    DeleteMediaResponse, GetMediaRequest, GetMediaResponse, ListMediaRequest,
    ListMediaResponse, MediaResponse, UpdateMediaRequest, UpdateMediaResponse,
};
use crate::auth::get_user_id;
use crate::db::DbError;
use crate::files::FileService;
use crate::model::Media;
use crate::CommerceService;

use super::{paginate, parse_uuid};

pub struct MediaService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    file_service: FileService,
    commerce_service: CommerceService,
}

impl MediaService {
    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        file_service: FileService,
        commerce_service: CommerceService,
    ) -> Self {
        Self {
            pool,
            verifier,
            file_service,
            commerce_service,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        file_service: FileService,
        commerce_service: CommerceService,
        max_size: usize,
    ) -> MediaServiceServer<Self> {
        MediaServiceServer::new(Self::new(
            pool,
            verifier,
            file_service,
            commerce_service,
        ))
        .max_decoding_message_size(max_size)
        .max_encoding_message_size(max_size)
    }

    fn to_response(
        &self,
        media: Media,
        data: Option<Vec<u8>>,
    ) -> MediaResponse {
        MediaResponse {
            media_id: media.media_id.to_string(),
            offer_ids: media
                .offer_ids
                .map(|ids| ids.into_iter().map(|id| id.to_string()).collect())
                .unwrap_or_default(),
            market_booth_id: media.market_booth_id.to_string(),
            user_id: media.user_id,
            created_at: media.created_at.timestamp(),
            updated_at: media.updated_at.timestamp(),
            name: media.name,
            data,
        }
    }

    fn decode_base64(image_string: &String) -> Result<Vec<u8>, Status> {
        base64::engine::general_purpose::STANDARD
            .decode(image_string)
            .map_err(|err| {
                tracing::log::error!("[MediaService.decode_base64]: {err}");
                Status::invalid_argument("image")
            })
    }

    fn build_data_url(
        user_id: &String,
        market_booth_id: &Uuid,
        media_id: &Uuid,
    ) -> String {
        format!("{user_id}/{market_booth_id}/{media_id}")
    }
}

#[async_trait]
impl media_service_server::MediaService for MediaService {
    async fn create_media(
        &self,
        request: Request<CreateMediaRequest>,
    ) -> Result<Response<CreateMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let CreateMediaRequest {
            market_booth_id,
            name,
            data,
        } = request.into_inner();

        let market_booth_uuid =
            parse_uuid(&market_booth_id, "market_booth_id")?;

        self.commerce_service
            .check_market_booth_and_owner(&market_booth_id, &user_id)
            .await?;

        let data = data.ok_or(Status::invalid_argument("data"))?;
        let data = Self::decode_base64(&data.data)?;

        let media_id = Uuid::new_v4();

        let data_url =
            Self::build_data_url(&user_id, &market_booth_uuid, &media_id);

        let mut conn = self.pool.get().await.map_err(DbError::from)?;
        let transaction = conn.transaction().await.map_err(DbError::from)?;
        let created_media = Media::create(
            &transaction,
            &media_id,
            &market_booth_uuid,
            &user_id,
            &name,
            &data_url,
        )
        .await?;
        self.file_service.put_file(&data_url, &data).await?;
        transaction.commit().await.map_err(DbError::from)?;

        Ok(Response::new(CreateMediaResponse {
            media: Some(self.to_response(created_media, None)),
        }))
    }

    async fn get_media(
        &self,
        request: Request<GetMediaRequest>,
    ) -> Result<Response<GetMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let GetMediaRequest { media_id } = request.into_inner();
        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        if found_media.user_id != user_id {
            return Err(Status::not_found(""));
        }

        let data_url = Self::build_data_url(
            &user_id,
            &found_media.market_booth_id,
            &media_uuid,
        );

        let data = self.file_service.get_file(&data_url).await?;

        Ok(Response::new(GetMediaResponse {
            media: Some(self.to_response(found_media, Some(data))),
        }))
    }

    async fn list_media(
        &self,
        request: Request<ListMediaRequest>,
    ) -> Result<Response<ListMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let ListMediaRequest {
            market_booth_id,
            pagination,
            order_by,
            filter,
        } = request.into_inner();

        let market_booth_id = parse_uuid(&market_booth_id, "market_booth_id")?;

        let (limit, offset, pagination) = paginate(pagination)?;

        let found_medias = Media::list(
            &self.pool,
            &market_booth_id,
            &user_id,
            limit,
            offset,
            None,
            None,
        )
        .await?;

        Ok(Response::new(ListMediaResponse {
            medias: found_medias
                .into_iter()
                .map(|m| self.to_response(m, None))
                .collect(),
            pagination: Some(pagination),
        }))
    }

    async fn update_media(
        &self,
        request: Request<UpdateMediaRequest>,
    ) -> Result<Response<UpdateMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let UpdateMediaRequest {
            media_id,
            name,
            data,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        let updated_media =
            Media::update(&self.pool, &media_uuid, &user_id, name).await?;

        if let Some(data) = data {
            let data = Self::decode_base64(&data.data)?;

            self.file_service
                .put_file(&found_media.data_url, &data)
                .await?;
        }

        Ok(Response::new(UpdateMediaResponse {
            media: Some(self.to_response(updated_media, None)),
        }))
    }

    async fn delete_media(
        &self,
        request: Request<DeleteMediaRequest>,
    ) -> Result<Response<DeleteMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let DeleteMediaRequest { media_id } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        let mut conn = self.pool.get().await.map_err(DbError::from)?;
        let transaction = conn.transaction().await.map_err(DbError::from)?;
        Media::begin_delete(&transaction, &media_uuid, &user_id).await?;
        self.file_service.remove_file(&found_media.data_url).await?;
        transaction.commit().await.map_err(DbError::from)?;

        Ok(Response::new(DeleteMediaResponse {}))
    }
}
