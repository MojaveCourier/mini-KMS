use chrono::{DateTime, Utc};
use serde::Serialize;

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
