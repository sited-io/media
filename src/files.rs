use s3::Bucket;
use tonic::Status;

#[derive(Debug, Clone)]
pub struct FileService {
    bucket: Bucket,
}

impl FileService {
    pub fn new(
        bucket_name: String,
        account_id: String,
        access_key_id: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            bucket: s3::Bucket::new(
                &bucket_name,
                s3::Region::R2 { account_id },
                s3::creds::Credentials {
                    access_key: Some(access_key_id),
                    secret_key: Some(secret_access_key),
                    security_token: None,
                    session_token: None,
                    expiration: None,
                },
            )
            .unwrap(),
        }
    }

    pub async fn put_file(
        &self,
        file_path: &String,
        file_data: &[u8],
    ) -> Result<(), Status> {
        self.bucket
            .put_object(file_path, file_data)
            .await
            .map_err(|err| {
                tracing::log::error!("[FileService.put_file]: {err}");
                Status::internal("")
            })?;

        Ok(())
    }

    pub async fn get_file(
        &self,
        file_path: &String,
    ) -> Result<Vec<u8>, Status> {
        let found_file =
            self.bucket.get_object(file_path).await.map_err(|err| {
                tracing::log::error!("{err}");
                Status::not_found("")
            })?;

        Ok(found_file.to_vec())
    }

    pub async fn remove_file(&self, file_path: &String) -> Result<(), Status> {
        self.bucket.delete_object(file_path).await.map_err(|err| {
            tracing::log::error!("{err}");
            Status::internal("")
        })?;

        Ok(())
    }
}
