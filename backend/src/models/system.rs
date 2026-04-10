use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemStatusDto {
    pub user_count: usize,
    pub device_count: usize,
    pub active_device_count: usize,
    pub keypack_count: usize,
}
