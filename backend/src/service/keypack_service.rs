use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::keypack::{CreateKeypackRequest, KeypackDto, UpdateKeypackRequest},
    repository::{audit_log_repository, device_repository, keypack_repository},
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

    pub async fn create_keypack(&self, actor_id: i64, payload: CreateKeypackRequest) -> Result<KeypackDto, AppError> {
        if payload.version.trim().is_empty() {
            return Err(AppError::bad_request("version is required"));
        }
        if !Self::valid_keypack_status(&payload.status) {
            return Err(AppError::bad_request("invalid keypack status"));
        }

        device_repository::find_by_id(&self.db, payload.device_id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::bad_request("device not found"))?;

        let keypack = keypack_repository::create(
            &self.db,
            payload.device_id,
            payload.version.trim().to_string(),
            payload.status,
        )
        .await
        .map_err(|_| AppError::internal("failed to create keypack"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "CREATE_KEYPACK",
            "keypack",
            Some(keypack.id),
            Some(format!("created keypack v{} for device {}", keypack.version, keypack.device_id)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        self.keypack_to_dto(keypack.id).await
    }

    pub async fn update_keypack(&self, actor_id: i64, id: i64, payload: UpdateKeypackRequest) -> Result<KeypackDto, AppError> {
        if payload.version.trim().is_empty() {
            return Err(AppError::bad_request("version is required"));
        }
        if !Self::valid_keypack_status(&payload.status) {
            return Err(AppError::bad_request("invalid keypack status"));
        }

        device_repository::find_by_id(&self.db, payload.device_id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::bad_request("device not found"))?;

        let keypack = keypack_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load keypack"))?
            .ok_or_else(|| AppError::not_found("keypack not found"))?;

        let updated = keypack_repository::update(
            &self.db,
            keypack,
            payload.device_id,
            payload.version.trim().to_string(),
            payload.status,
        )
        .await
        .map_err(|_| AppError::internal("failed to update keypack"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "UPDATE_KEYPACK",
            "keypack",
            Some(id),
            Some(format!("updated keypack v{} (device {})", updated.version, updated.device_id)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        self.keypack_to_dto(updated.id).await
    }

    pub async fn delete_keypack(&self, actor_id: i64, id: i64) -> Result<(), AppError> {
        let keypack = keypack_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load keypack"))?
            .ok_or_else(|| AppError::not_found("keypack not found"))?;

        let detail = format!("deleted keypack v{} (device {})", keypack.version, keypack.device_id);
        keypack_repository::delete(&self.db, keypack)
            .await
            .map_err(|_| AppError::internal("failed to delete keypack"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "DELETE_KEYPACK",
            "keypack",
            Some(id),
            Some(detail),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(())
    }

    async fn keypack_to_dto(&self, id: i64) -> Result<KeypackDto, AppError> {
        let keypack = keypack_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load keypack"))?
            .ok_or_else(|| AppError::not_found("keypack not found"))?;

        let device = device_repository::find_by_id(&self.db, keypack.device_id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::internal("keypack device missing"))?;

        Ok(KeypackDto {
            id: keypack.id,
            device_id: keypack.device_id,
            device_name: device.name,
            device_serial: device.serial,
            version: keypack.version,
            status: keypack.status,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(keypack.created_at, Utc),
        })
    }

    fn valid_keypack_status(status: &str) -> bool {
        matches!(status, "draft" | "issued" | "active" | "revoked")
    }
}
