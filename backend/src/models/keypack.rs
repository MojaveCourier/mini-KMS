use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct KeypackDto {
    pub id: i64,
    pub device_id: i64,
    pub device_name: String,
    pub device_serial: String,
    pub version: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKeypackRequest {
    pub device_id: i64,
    pub version: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKeypackRequest {
    pub device_id: i64,
    pub version: String,
    pub status: String,
}
