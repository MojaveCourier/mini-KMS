use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entity::users;

#[derive(Debug, Serialize, Clone)]
pub struct UserDto {
    pub id: i64,
    pub account: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub account: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub account: String,
    pub password: Option<String>,
    pub role: String,
}

impl UserDto {
    pub fn from_db(value: &users::Model) -> Self {
        Self {
            id: value.id,
            account: value.account.clone(),
            role: value.role.clone(),
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(value.created_at, Utc),
        }
    }
}
