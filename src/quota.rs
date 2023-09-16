use deadpool_postgres::Pool;
use tonic::Status;

use crate::model::{Media, MediaQuota};

pub struct QuotaService {
    pool: Pool,
    default_user_quota_mib: u64,
}

impl QuotaService {
    pub fn new(pool: Pool, default_user_quota_mib: u64) -> Self {
        Self {
            pool,
            default_user_quota_mib,
        }
    }

    fn quota_reached(total_bytes: u64, max_size_mib: u64) -> bool {
        total_bytes > max_size_mib * 1024 * 1024
    }

    async fn ensure_user_quota(
        &self,
        user_id: &String,
    ) -> Result<MediaQuota, Status> {
        let found_quota = MediaQuota::get(&self.pool, user_id).await?;

        match found_quota {
            Some(q) => Ok(q),
            None => {
                let new_quota = MediaQuota::create(
                    &self.pool,
                    user_id,
                    self.default_user_quota_mib,
                )
                .await?;

                Ok(new_quota)
            }
        }
    }

    pub async fn check_quota(&self, user_id: &String) -> Result<(), Status> {
        let user_quota = self.ensure_user_quota(user_id).await?;
        let found_medias =
            Media::list_all_for_user(&self.pool, user_id).await?;

        let total_bytes: u64 = found_medias.iter().map(|m| m.size_bytes).sum();

        if Self::quota_reached(total_bytes, user_quota.max_size_mib) {
            Err(Status::out_of_range("quota"))
        } else {
            Ok(())
        }
    }
}
