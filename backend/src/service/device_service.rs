use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::device::{CreateDeviceRequest, DeviceRecord, UpdateDeviceRequest},
    repository::{audit_log_repository, device_repository, keypack_repository},
};

#[derive(Clone)]
pub struct DeviceService {
    db: DatabaseConnection,
}

impl DeviceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_devices(&self) -> Result<Vec<DeviceRecord>, AppError> {
        let devices = device_repository::list(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to load devices"))?;

        Ok(devices
            .into_iter()
            .map(|device| DeviceRecord {
                id: device.id,
                serial: device.serial,
                name: device.name,
                status: device.status,
                last_seen_at: device
                    .last_seen_at
                    .map(|value| DateTime::<Utc>::from_naive_utc_and_offset(value, Utc)),
                created_at: DateTime::<Utc>::from_naive_utc_and_offset(device.created_at, Utc),
            })
            .collect())
    }

    pub async fn get_device(&self, id: i64) -> Result<DeviceRecord, AppError> {
        let device = device_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::not_found("device not found"))?;

        Ok(DeviceRecord {
            id: device.id,
            serial: device.serial,
            name: device.name,
            status: device.status,
            last_seen_at: device
                .last_seen_at
                .map(|value| DateTime::<Utc>::from_naive_utc_and_offset(value, Utc)),
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(device.created_at, Utc),
        })
    }

    pub async fn create_device(&self, actor_id: i64, payload: CreateDeviceRequest) -> Result<DeviceRecord, AppError> {
        if payload.serial.trim().is_empty() || payload.name.trim().is_empty() {
            return Err(AppError::bad_request("serial and name are required"));
        }
        if !Self::valid_device_status(&payload.status) {
            return Err(AppError::bad_request("invalid device status"));
        }

        if device_repository::find_by_serial(&self.db, payload.serial.trim())
            .await
            .map_err(|_| AppError::internal("failed to check serial"))?
            .is_some()
        {
            return Err(AppError::bad_request("serial already exists"));
        }

        let last_seen = payload.last_seen_at.map(|dt| dt.naive_utc());
        let device = device_repository::create(
            &self.db,
            payload.serial.trim().to_string(),
            payload.name.trim().to_string(),
            payload.status,
            last_seen,
        )
        .await
        .map_err(|_| AppError::internal("failed to create device"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "CREATE_DEVICE",
            "device",
            Some(device.id),
            Some(format!("created device {}", device.serial)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        self.get_device(device.id).await
    }

    pub async fn update_device(&self, actor_id: i64, id: i64, payload: UpdateDeviceRequest) -> Result<DeviceRecord, AppError> {
        if payload.serial.trim().is_empty() || payload.name.trim().is_empty() {
            return Err(AppError::bad_request("serial and name are required"));
        }
        if !Self::valid_device_status(&payload.status) {
            return Err(AppError::bad_request("invalid device status"));
        }

        if let Some(existing) = device_repository::find_by_serial(&self.db, payload.serial.trim())
            .await
            .map_err(|_| AppError::internal("failed to check serial"))?
            && existing.id != id
        {
            return Err(AppError::bad_request("serial already exists"));
        }

        let device = device_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::not_found("device not found"))?;

        let last_seen = payload.last_seen_at.map(|dt| dt.naive_utc());
        let updated = device_repository::update(
            &self.db,
            device,
            payload.serial.trim().to_string(),
            payload.name.trim().to_string(),
            payload.status,
            last_seen,
        )
        .await
        .map_err(|_| AppError::internal("failed to update device"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "UPDATE_DEVICE",
            "device",
            Some(id),
            Some(format!("updated device {}", updated.serial)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        self.get_device(updated.id).await
    }

    pub async fn delete_device(&self, actor_id: i64, id: i64) -> Result<(), AppError> {
        let count = keypack_repository::count_by_device(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to count keypacks"))?;
        if count > 0 {
            return Err(AppError::bad_request("device has keypacks; remove keypacks first"));
        }

        let device = device_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to load device"))?
            .ok_or_else(|| AppError::not_found("device not found"))?;

        let serial = device.serial.clone();
        device_repository::delete(&self.db, device)
            .await
            .map_err(|_| AppError::internal("failed to delete device"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "DELETE_DEVICE",
            "device",
            Some(id),
            Some(format!("deleted device {}", serial)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(())
    }

    fn valid_device_status(status: &str) -> bool {
        matches!(status, "active" | "inactive")
    }
}
