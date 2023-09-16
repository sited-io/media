use aws_credential_types::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use tonic::Status;

#[derive(Debug, Clone)]
pub struct FileService {
    client: Client,
    bucket_name: String,
}

impl FileService {
    pub async fn new(
        bucket_name: String,
        bucket_endpoint: String,
        access_key_id: String,
        secret_access_key: String,
    ) -> Self {
        let credentials =
            Credentials::from_keys(access_key_id, secret_access_key, None);

        let config = aws_config::from_env()
            .credentials_provider(credentials)
            .region(Region::new("auto"))
            .endpoint_url(bucket_endpoint)
            .load()
            .await;

        let client = Client::new(&config);

        Self {
            bucket_name,
            client,
        }
    }

    pub async fn put_file(
        &self,
        file_path: &String,
        file_data: &[u8],
        content_type: &String,
    ) -> Result<(), Status> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .body(ByteStream::from(file_data.to_vec()))
            .content_type(content_type)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!("[FileService.put_file]: {err}");
                Status::internal("")
            })?;

        Ok(())
    }

    /// Returns `upload_id`
    pub async fn initiate_multipart_upload(
        &self,
        file_path: &String,
        content_type: &String,
    ) -> Result<String, Status> {
        let response = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket_name)
            .key(file_path)
            .content_type(content_type)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[FileService.initiate_multipart_upload]: {err}"
                );
                Status::internal("")
            })?;

        if let Some(upload_id) = response.upload_id {
            Ok(upload_id)
        } else {
            Err(Status::data_loss("upload_id"))
        }
    }

    /// Returns `e_tag`
    pub async fn put_multipart_chunk(
        &self,
        file_path: &String,
        upload_id: &String,
        part_number: u32,
        file_data: &[u8],
    ) -> Result<String, Status> {
        let part_number = part_number
            .try_into()
            .map_err(|_| Status::invalid_argument("part_number"))?;

        let part = self
            .client
            .upload_part()
            .bucket(&self.bucket_name)
            .key(file_path)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(ByteStream::from(file_data.to_vec()))
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[FileService.put_multipart_chunk]: {err}"
                );
                Status::internal("")
            })?;

        Ok(part.e_tag.unwrap_or_default())
    }

    pub async fn complete_multipart_upload(
        &self,
        file_path: &String,
        upload_id: &String,
        parts: Vec<CompletedPart>,
    ) -> Result<(), Status> {
        let completed_multipart_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(file_path)
            .upload_id(upload_id)
            .multipart_upload(completed_multipart_upload)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[FileService.complete_multipart_upload]: {err}"
                );
                Status::internal("")
            })?;

        Ok(())
    }

    pub async fn abort_multipart_upload(
        &self,
        file_path: &String,
        upload_id: &String,
    ) -> Result<(), Status> {
        self.client
            .abort_multipart_upload()
            .bucket(&self.bucket_name)
            .key(file_path)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[FileService.abort_multipart_upload]: {err}"
                );
                Status::internal("")
            })?;

        Ok(())
    }

    pub async fn get_file(
        &self,
        file_path: &String,
    ) -> Result<Vec<u8>, Status> {
        let found_file = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!("[FileService.get_file]: {err}");
                Status::not_found("")
            })?;

        let data = found_file.body.collect().await.map_err(|err| {
            tracing::log::error!("[FileService.get_file]: {err}");
            Status::internal("")
        })?;

        Ok(data.to_vec())
    }

    pub async fn remove_file(&self, file_path: &String) -> Result<(), Status> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!("[FileService.remove_file]: {err}");
                Status::internal("")
            })?;

        Ok(())
    }
}
