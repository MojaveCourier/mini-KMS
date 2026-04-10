use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;

use crate::{error::AppError, models::device::DeviceRecord, repository::device_repository};

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
}
