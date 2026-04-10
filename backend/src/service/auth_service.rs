use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::{
        auth::{AuthClaims, LoginRequest, LoginResponse},
        user::UserDto,
    },
    repository::{audit_log_repository, user_repository},
};

#[derive(Clone)]
pub struct AuthService {
    db: DatabaseConnection,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::encode_b64(uuid::Uuid::new_v4().as_bytes())
            .map_err(|_| AppError::internal("failed to create password salt"))?;
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|value| value.to_string())
            .map_err(|_| AppError::internal("failed to hash password"))
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| AppError::internal("invalid stored password hash"))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn login(&self, payload: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = user_repository::find_by_account(&self.db, &payload.account)
            .await
            .map_err(|_| AppError::internal("failed to query user"))?
            .ok_or(AppError::Unauthorized)?;

        if !Self::verify_password(&payload.password, &user.password_hash)? {
            return Err(AppError::Unauthorized);
        }

        let claims = AuthClaims {
            sub: user.id,
            account: user.account.clone(),
            role: user.role.clone(),
            exp: (Utc::now() + Duration::hours(8)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::internal("failed to create token"))?;

        audit_log_repository::append(
            &self.db,
            Some(user.id),
            "LOGIN",
            "session",
            None,
            Some(format!("{} logged in", user.account)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(LoginResponse {
            token,
            user: UserDto::from_db(&user),
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<AuthClaims, AppError> {
        decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|value| value.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}
