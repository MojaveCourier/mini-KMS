use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::system::SystemStatusDto,
    repository::{device_repository, keypack_repository, user_repository},
};

#[derive(Clone)]
pub struct SystemService {
    db: DatabaseConnection,
}

impl SystemService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_status(&self) -> Result<SystemStatusDto, AppError> {
        let user_count = user_repository::count(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to count users"))?;
        let device_count = device_repository::count(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to count devices"))?;
        let active_device_count = device_repository::count_active(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to count active devices"))?;
        let keypack_count = keypack_repository::count(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to count keypacks"))?;

        Ok(SystemStatusDto {
            user_count: user_count as usize,
            device_count: device_count as usize,
            active_device_count: active_device_count as usize,
            keypack_count: keypack_count as usize,
        })
    }
}
