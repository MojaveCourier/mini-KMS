use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::keypack::KeypackDto,
    repository::keypack_repository,
};

#[derive(Clone)]
pub struct KeypackService {
    db: DatabaseConnection,
}

impl KeypackService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_keypacks(&self) -> Result<Vec<KeypackDto>, AppError> {
        let keypacks = keypack_repository::list_with_device(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to load keypacks"))?
            .into_iter()
            .filter_map(|(keypack, device)| {
                device.map(|device| KeypackDto {
                    id: keypack.id,
                    device_id: keypack.device_id,
                    device_name: device.name,
                    device_serial: device.serial,
                    version: keypack.version,
                    status: keypack.status,
                    created_at: DateTime::<Utc>::from_naive_utc_and_offset(keypack.created_at, Utc),
                })
            })
            .collect();

        Ok(keypacks)
    }
}
