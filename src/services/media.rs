use aws_sdk_s3::types::CompletedPart;
use deadpool_postgres::Pool;
use jwtk::jwk::RemoteJwksVerifier;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

use crate::api::peoplesmarkets::media::v1::media_service_server::{
    self, MediaServiceServer,
};
use crate::api::peoplesmarkets::media::v1::{
    AddMediaToOfferRequest, AddMediaToOfferResponse,
    CompleteMultipartUploadRequest, CompleteMultipartUploadResponse,
    CreateMediaRequest, CreateMediaResponse, DeleteMediaRequest,
    DeleteMediaResponse, GetMediaRequest, GetMediaResponse,
    InitiateMultipartUploadRequest, InitiateMultipartUploadResponse,
    ListMediaRequest, ListMediaResponse, MediaResponse, Part,
    PutMultipartChunkRequest, PutMultipartChunkResponse,
    RemoveMediaFromOfferRequest, RemoveMediaFromOfferResponse,
    UpdateMediaRequest, UpdateMediaResponse,
};
use crate::auth::get_user_id;
use crate::db::DbError;
use crate::files::FileService;
use crate::model::{Media, MediaOffer};
use crate::{CommerceService, QuotaService};

use super::{paginate, parse_uuid};

pub struct MediaService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    file_service: FileService,
    commerce_service: CommerceService,
    quota_service: QuotaService,
}

impl MediaService {
    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        file_service: FileService,
        commerce_service: CommerceService,
        quota_service: QuotaService,
    ) -> Self {
        Self {
            pool,
            verifier,
            file_service,
            commerce_service,
            quota_service,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        file_service: FileService,
        commerce_service: CommerceService,
        quota_service: QuotaService,
        max_message_size_bytes: usize,
    ) -> MediaServiceServer<Self> {
        MediaServiceServer::new(Self::new(
            pool,
            verifier,
            file_service,
            commerce_service,
            quota_service,
        ))
        .max_decoding_message_size(max_message_size_bytes)
        .max_encoding_message_size(max_message_size_bytes)
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

    fn build_file_path(
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
            file,
        } = request.into_inner();

        let market_booth_uuid =
            parse_uuid(&market_booth_id, "market_booth_id")?;

        self.quota_service.check_quota(&user_id).await?;

        self.commerce_service
            .check_market_booth_and_owner(&market_booth_id, &user_id)
            .await?;

        let media_id = Uuid::new_v4();

        let file_path =
            Self::build_file_path(&user_id, &market_booth_uuid, &media_id);

        let mut conn = self.pool.get().await.map_err(DbError::from)?;
        let transaction = conn.transaction().await.map_err(DbError::from)?;

        let size = file
            .as_ref()
            .map(|f| f.data.len())
            .unwrap_or(0)
            .try_into()
            .map_err(|_| Status::internal(""))?;

        let created_media = Media::create(
            &transaction,
            &media_id,
            &market_booth_uuid,
            &user_id,
            &name,
            &file_path,
            size,
        )
        .await?;

        if let Some(file) = file {
            self.file_service
                .put_file(&file_path, &file.data, &file.content_type)
                .await?;
        }

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

        let file_path = Self::build_file_path(
            &user_id,
            &found_media.market_booth_id,
            &media_uuid,
        );

        let data = self.file_service.get_file(&file_path).await?;

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

        let filter = filter.map(|f| (f.field(), f.query));

        let order_by = order_by.map(|o| (o.field(), o.direction()));

        let found_medias = Media::list(
            &self.pool,
            &market_booth_id,
            &user_id,
            limit,
            offset,
            filter,
            order_by,
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
            file,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        let new_size =
            file.as_ref().and_then(|f| i64::try_from(f.data.len()).ok());

        let updated_media =
            Media::update(&self.pool, &media_uuid, &user_id, name, new_size)
                .await?;

        if let Some(file) = file {
            self.file_service
                .put_file(&found_media.data_url, &file.data, &file.content_type)
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

    async fn initiate_multipart_upload(
        &self,
        request: Request<InitiateMultipartUploadRequest>,
    ) -> Result<Response<InitiateMultipartUploadResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let InitiateMultipartUploadRequest {
            media_id,
            content_type,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        self.quota_service.check_quota(&user_id).await?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        if found_media.user_id != user_id {
            return Err(Status::not_found(&media_id));
        }

        let upload_id = self
            .file_service
            .initiate_multipart_upload(&found_media.data_url, &content_type)
            .await?;

        Ok(Response::new(InitiateMultipartUploadResponse {
            key: found_media.data_url,
            upload_id,
        }))
    }

    async fn put_multipart_chunk(
        &self,
        request: Request<PutMultipartChunkRequest>,
    ) -> Result<Response<PutMultipartChunkResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let PutMultipartChunkRequest {
            media_id,
            upload_id,
            part_number,
            chunk,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let additional_size =
            i64::try_from(chunk.len()).map_err(|_| Status::internal(""))?;

        // user_id check is done implicitly in add_size
        let found_media =
            Media::add_size(&self.pool, &media_uuid, &user_id, additional_size)
                .await?;

        if self.quota_service.check_quota(&user_id).await.is_err() {
            Media::update(&self.pool, &media_uuid, &user_id, None, Some(0))
                .await?;

            self.file_service
                .abort_multipart_upload(&found_media.data_url, &upload_id)
                .await?;

            return Err(Status::aborted("quota reached"));
        }

        let etag = self
            .file_service
            .put_multipart_chunk(
                &found_media.data_url,
                &upload_id,
                part_number,
                &chunk,
            )
            .await?;

        Ok(Response::new(PutMultipartChunkResponse {
            part: Some(Part { part_number, etag }),
        }))
    }

    async fn complete_multipart_upload(
        &self,
        request: Request<CompleteMultipartUploadRequest>,
    ) -> Result<Response<CompleteMultipartUploadResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let CompleteMultipartUploadRequest {
            media_id,
            upload_id,
            parts,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media = Media::get(&self.pool, &media_uuid)
            .await?
            .ok_or(Status::not_found(&media_id))?;

        if found_media.user_id != user_id {
            return Err(Status::not_found(&media_id));
        }

        let parts = parts
            .into_iter()
            .map(|p| {
                CompletedPart::builder()
                    .e_tag(p.etag)
                    .part_number(p.part_number.try_into().unwrap())
                    .build()
            })
            .collect();

        self.file_service
            .complete_multipart_upload(&found_media.data_url, &upload_id, parts)
            .await?;

        Ok(Response::new(CompleteMultipartUploadResponse {}))
    }

    async fn add_media_to_offer(
        &self,
        request: Request<AddMediaToOfferRequest>,
    ) -> Result<Response<AddMediaToOfferResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let AddMediaToOfferRequest { media_id, offer_id } =
            request.into_inner();

        let media_id = parse_uuid(&media_id, "media_id")?;
        let offer_uuid = parse_uuid(&offer_id, "media_id")?;

        self.commerce_service
            .check_offer_and_owner(&offer_id, &user_id)
            .await?;

        Media::get_for_user(&self.pool, &media_id, &user_id)
            .await?
            .ok_or(Status::not_found("media"))?;

        MediaOffer::create(&self.pool, &media_id, &offer_uuid, &user_id)
            .await?;

        Ok(Response::new(AddMediaToOfferResponse {}))
    }

    async fn remove_media_from_offer(
        &self,
        request: Request<RemoveMediaFromOfferRequest>,
    ) -> Result<Response<RemoveMediaFromOfferResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let RemoveMediaFromOfferRequest { media_id, offer_id } =
            request.into_inner();

        let media_id = parse_uuid(&media_id, "media_id")?;
        let offer_id = parse_uuid(&offer_id, "offer_id")?;

        MediaOffer::delete(&self.pool, &media_id, &offer_id, &user_id).await?;

        Ok(Response::new(RemoveMediaFromOfferResponse {}))
    }
}
