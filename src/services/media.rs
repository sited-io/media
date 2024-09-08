use std::cmp::Ordering;

use aws_sdk_s3::types::CompletedPart;
use deadpool_postgres::Pool;
use jwtk::jwk::RemoteJwksVerifier;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

use crate::api::sited_io::media::v1::media_service_server::{
    self, MediaServiceServer,
};
use crate::api::sited_io::media::v1::{
    AddMediaToOfferRequest, AddMediaToOfferResponse,
    CompleteMultipartUploadRequest, CompleteMultipartUploadResponse,
    CreateMediaRequest, CreateMediaResponse, DeleteMediaRequest,
    DeleteMediaResponse, DownloadMediaRequest, DownloadMediaResponse,
    GetMediaRequest, GetMediaResponse, InitiateMultipartUploadRequest,
    InitiateMultipartUploadResponse, ListAccessibleMediaRequest,
    ListAccessibleMediaResponse, ListMediaRequest, ListMediaResponse,
    MediaResponse, Part, PutMultipartChunkRequest, PutMultipartChunkResponse,
    RemoveMediaFromOfferRequest, RemoveMediaFromOfferResponse,
    UpdateMediaOfferOrderingRequest, UpdateMediaOfferOrderingResponse,
    UpdateMediaRequest, UpdateMediaResponse,
};
use crate::auth::get_user_id;
use crate::db::DbError;
use crate::files::FileService;
use crate::model::{Media, MediaOffer, SubOffer, SubShop};
use crate::QuotaService;

use super::{get_limit_offset_from_pagination, parse_uuid};

pub struct MediaService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    file_service: FileService,
    quota_service: QuotaService,
}

impl MediaService {
    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        file_service: FileService,
        quota_service: QuotaService,
        max_message_size_bytes: usize,
    ) -> MediaServiceServer<Self> {
        MediaServiceServer::new(Self {
            pool,
            verifier,
            file_service,
            quota_service,
        })
        .max_decoding_message_size(max_message_size_bytes)
        .max_encoding_message_size(max_message_size_bytes)
    }

    fn to_response(&self, media: Media) -> MediaResponse {
        MediaResponse {
            media_id: media.media_id.to_string(),
            offer_ids: media
                .offer_ids
                .map(|ids| ids.into_iter().map(|id| id.to_string()).collect())
                .unwrap_or_default(),
            shop_id: media.shop_id.to_string(),
            user_id: media.user_id,
            created_at: media.created_at.timestamp(),
            updated_at: media.updated_at.timestamp(),
            name: media.name,
            file_name: media.file_name,
            ordering: media.ordering,
        }
    }

    fn build_file_path(
        user_id: &String,
        shop_id: &Uuid,
        media_id: &Uuid,
    ) -> String {
        format!("{user_id}/{shop_id}/{media_id}")
    }

    async fn check_shop_and_owner(
        &self,
        shop_id: &Uuid,
        user_id: &String,
    ) -> Result<SubShop, Status> {
        SubShop::get_for_user(&self.pool, shop_id, user_id)
            .await
            .map_err(|_| Status::internal(""))?
            .ok_or(Status::not_found("user is not owner of this shop"))
    }

    async fn check_offer_and_owner(
        &self,
        offer_id: &Uuid,
        user_id: &String,
    ) -> Result<SubOffer, Status> {
        SubOffer::get_for_owner(&self.pool, offer_id, user_id)
            .await
            .map_err(|_| Status::internal(""))?
            .ok_or(Status::not_found("user is not owner of this offer"))
    }
}

#[async_trait]
impl media_service_server::MediaService for MediaService {
    async fn create_media(
        &self,
        request: Request<CreateMediaRequest>,
    ) -> Result<Response<CreateMediaResponse>, Status> {
        let metadata = request.metadata().clone();

        let user_id = get_user_id(&metadata, &self.verifier).await?;

        let CreateMediaRequest {
            shop_id,
            name,
            file,
            file_name,
        } = request.into_inner();

        let shop_id = parse_uuid(&shop_id, "shop_id")?;

        self.quota_service.check_quota(&user_id).await?;

        self.check_shop_and_owner(&shop_id, &user_id).await?;

        let media_id = Uuid::new_v4();

        let file_path = Self::build_file_path(&user_id, &shop_id, &media_id);

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
            &shop_id,
            &user_id,
            &name,
            &file_path,
            size,
            &file_name,
        )
        .await?;

        if let Some(file) = file {
            self.file_service
                .put_file(&file_path, &file.data, &file.content_type)
                .await?;
        }

        transaction.commit().await.map_err(DbError::from)?;

        Ok(Response::new(CreateMediaResponse {
            media: Some(self.to_response(created_media)),
        }))
    }

    async fn get_media(
        &self,
        request: Request<GetMediaRequest>,
    ) -> Result<Response<GetMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let GetMediaRequest { media_id } = request.into_inner();
        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media =
            Media::get_for_owner(&self.pool, &media_uuid, &user_id)
                .await?
                .ok_or(Status::not_found(&media_id))?;

        Ok(Response::new(GetMediaResponse {
            media: Some(self.to_response(found_media)),
        }))
    }

    async fn download_media(
        &self,
        request: Request<DownloadMediaRequest>,
    ) -> Result<Response<DownloadMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let DownloadMediaRequest { media_id } = request.into_inner();
        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media =
            Media::get_accessible(&self.pool, &media_uuid, &user_id)
                .await?
                .ok_or(Status::not_found(&media_id))?;

        let file_path = Self::build_file_path(
            &found_media.user_id,
            &found_media.shop_id,
            &found_media.media_id,
        );

        let download_url = self
            .file_service
            .get_presigned_url(&file_path, &found_media.file_name)
            .await?;

        Ok(Response::new(DownloadMediaResponse { download_url }))
    }

    async fn list_media(
        &self,
        request: Request<ListMediaRequest>,
    ) -> Result<Response<ListMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let ListMediaRequest {
            shop_id,
            pagination,
            order_by,
            filter,
        } = request.into_inner();

        let shop_id = parse_uuid(&shop_id, "shop_id")?;

        let (limit, offset, mut pagination) =
            get_limit_offset_from_pagination(pagination)?;

        let filter = filter.map(|f| (f.field(), f.query));

        let order_by = order_by.map(|o| (o.field(), o.direction()));

        let (found_medias, count) = Media::list(
            &self.pool,
            &shop_id,
            &user_id,
            limit.into(),
            offset.into(),
            filter,
            order_by,
        )
        .await?;

        pagination.total_elements = count.try_into().map_err(|_| {
            Status::internal("Could not convert 'count' from i64 to u32")
        })?;

        Ok(Response::new(ListMediaResponse {
            medias: found_medias
                .into_iter()
                .map(|m| self.to_response(m))
                .collect(),
            pagination: Some(pagination),
        }))
    }

    async fn list_accessible_media(
        &self,
        request: Request<ListAccessibleMediaRequest>,
    ) -> Result<Response<ListAccessibleMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await;

        let ListAccessibleMediaRequest {
            pagination,
            order_by,
            filter,
        } = request.into_inner();

        let (limit, offset, mut pagination) =
            get_limit_offset_from_pagination(pagination)?;

        let filter = filter.map(|f| (f.field(), f.query));

        let order_by = order_by.map(|o| (o.field(), o.direction()));

        let (found_medias, count) = match user_id {
            Ok(user_id) => {
                Media::list_accessible(
                    &self.pool,
                    &user_id,
                    limit.into(),
                    offset.into(),
                    filter,
                    order_by,
                )
                .await?
            }
            Err(_) => (vec![], 0),
        };

        pagination.total_elements = count.try_into().map_err(|_| {
            Status::internal("Could not convert 'count' from i64 to u32")
        })?;

        Ok(Response::new(ListAccessibleMediaResponse {
            medias: found_medias
                .into_iter()
                .map(|m| self.to_response(m))
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
            file_name,
        } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media =
            Media::get_for_owner(&self.pool, &media_uuid, &user_id)
                .await?
                .ok_or(Status::not_found(&media_id))?;

        let new_size =
            file.as_ref().and_then(|f| i64::try_from(f.data.len()).ok());

        let updated_media = Media::update(
            &self.pool,
            &media_uuid,
            &user_id,
            name,
            new_size,
            file_name,
        )
        .await?;

        if let Some(file) = file {
            self.file_service
                .put_file(&found_media.data_url, &file.data, &file.content_type)
                .await?;
        }

        Ok(Response::new(UpdateMediaResponse {
            media: Some(self.to_response(updated_media)),
        }))
    }

    async fn delete_media(
        &self,
        request: Request<DeleteMediaRequest>,
    ) -> Result<Response<DeleteMediaResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let DeleteMediaRequest { media_id } = request.into_inner();

        let media_uuid = parse_uuid(&media_id, "media_id")?;

        let found_media =
            Media::get_for_owner(&self.pool, &media_uuid, &user_id)
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

        let found_media =
            Media::get_for_owner(&self.pool, &media_uuid, &user_id)
                .await?
                .ok_or(Status::not_found(&media_id))?;

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
            self.file_service
                .abort_multipart_upload(&found_media.data_url, &upload_id)
                .await?;

            Media::delete(&self.pool, &media_uuid, &user_id).await?;

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

        let found_media =
            Media::get_for_owner(&self.pool, &media_uuid, &user_id)
                .await?
                .ok_or(Status::not_found(&media_id))?;

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
        let metadata = request.metadata().clone();

        let user_id = get_user_id(&metadata, &self.verifier).await?;

        let AddMediaToOfferRequest {
            media_id,
            offer_id,
            ordering,
        } = request.into_inner();

        let media_id = parse_uuid(&media_id, "media_id")?;
        let offer_id = parse_uuid(&offer_id, "media_id")?;

        // Check if user is owner of the offer
        self.check_offer_and_owner(&offer_id, &user_id).await?;

        // Check if user is owner of media
        Media::get_for_owner(&self.pool, &media_id, &user_id)
            .await?
            .ok_or(Status::not_found(media_id))?;

        let ord = match ordering {
            Some(o) => o,
            None => {
                let highest = MediaOffer::get_highest_ordering(
                    &self.pool, &offer_id, &user_id,
                )
                .await?;

                highest + 1
            }
        };

        MediaOffer::create(&self.pool, &media_id, &offer_id, &user_id, ord)
            .await?;

        Ok(Response::new(AddMediaToOfferResponse {}))
    }

    async fn update_media_offer_ordering(
        &self,
        request: Request<UpdateMediaOfferOrderingRequest>,
    ) -> Result<Response<UpdateMediaOfferOrderingResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let UpdateMediaOfferOrderingRequest {
            media_id,
            offer_id,
            ordering,
        } = request.into_inner();

        let media_id = parse_uuid(&media_id, "media_id")?;
        let offer_id = parse_uuid(&offer_id, "offer_id")?;

        let found_media_offer =
            MediaOffer::get(&self.pool, &media_id, &offer_id)
                .await?
                .ok_or_else(|| Status::not_found(""))?;

        let old_ordering = found_media_offer.ordering;

        let mut found_media_offers =
            MediaOffer::list(&self.pool, &user_id, &offer_id).await?;

        // update media_offer with new_ordering
        MediaOffer::update_ordering(
            &self.pool, &media_id, &offer_id, &user_id, ordering,
        )
        .await?;

        match old_ordering.cmp(&ordering) {
            Ordering::Less => {
                // decrement [old_ordering + 1, new_ordering] by 1
                found_media_offers.retain(|m| {
                    m.ordering >= (old_ordering + 1) && m.ordering <= ordering
                });
                for m in found_media_offers {
                    MediaOffer::update_ordering(
                        &self.pool,
                        &m.media_id,
                        &m.offer_id,
                        &user_id,
                        m.ordering - 1,
                    )
                    .await?;
                }
            }
            Ordering::Greater => {
                // increment [new_ordering, old_ordering -1] by 1
                found_media_offers.retain(|m| {
                    m.ordering >= ordering && m.ordering <= (old_ordering - 1)
                });
                for m in found_media_offers {
                    MediaOffer::update_ordering(
                        &self.pool,
                        &m.media_id,
                        &m.offer_id,
                        &user_id,
                        m.ordering + 1,
                    )
                    .await?;
                }
            }
            Ordering::Equal => {}
        }

        Ok(Response::new(UpdateMediaOfferOrderingResponse {}))
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
