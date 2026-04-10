use sea_orm::DatabaseConnection;

use crate::{
    error::AppError,
    models::{
        user::{CreateUserRequest, UpdateUserRequest, UserDto},
    },
    repository::{audit_log_repository, user_repository},
    service::auth_service::AuthService,
};

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_users(&self) -> Result<Vec<UserDto>, AppError> {
        let users = user_repository::list(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to list users"))?;
        Ok(users.iter().map(UserDto::from_db).collect())
    }

    pub async fn create_user(&self, actor_id: i64, payload: CreateUserRequest) -> Result<UserDto, AppError> {
        if payload.account.trim().is_empty() || payload.password.trim().is_empty() || payload.role.trim().is_empty() {
            return Err(AppError::bad_request("account, password and role are required"));
        }

        if user_repository::find_by_account(&self.db, &payload.account)
            .await
            .map_err(|_| AppError::internal("failed to check account"))?
            .is_some()
        {
            return Err(AppError::bad_request("account already exists"));
        }

        let user = user_repository::create(
            &self.db,
            payload.account,
            AuthService::hash_password(&payload.password)?,
            payload.role,
        )
        .await
        .map_err(|_| AppError::internal("failed to create user"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "CREATE_USER",
            "user",
            Some(user.id),
            Some(format!("created user {}", user.account)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(UserDto::from_db(&user))
    }

    pub async fn update_user(&self, actor_id: i64, id: i64, payload: UpdateUserRequest) -> Result<UserDto, AppError> {
        if payload.account.trim().is_empty() || payload.role.trim().is_empty() {
            return Err(AppError::bad_request("account and role are required"));
        }

        if let Some(existing) = user_repository::find_by_account(&self.db, &payload.account)
            .await
            .map_err(|_| AppError::internal("failed to check account"))?
            && existing.id != id
        {
            return Err(AppError::bad_request("account already exists"));
        };

        let user = user_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to query user"))?
            .ok_or_else(|| AppError::not_found("user not found"))?;

        let password_hash = match payload.password {
            Some(password) if !password.trim().is_empty() => Some(AuthService::hash_password(&password)?),
            _ => None,
        };

        let updated = user_repository::update(
            &self.db,
            user,
            Some(payload.account),
            password_hash,
            Some(payload.role),
        )
        .await
        .map_err(|_| AppError::internal("failed to update user"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "UPDATE_USER",
            "user",
            Some(id),
            Some(format!("updated user {}", updated.account)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(UserDto::from_db(&updated))
    }

    pub async fn delete_user(&self, actor_id: i64, id: i64) -> Result<(), AppError> {
        if actor_id == id {
            return Err(AppError::bad_request("cannot delete current login user"));
        }

        let user = user_repository::find_by_id(&self.db, id)
            .await
            .map_err(|_| AppError::internal("failed to query user"))?
            .ok_or_else(|| AppError::not_found("user not found"))?;

        let account = user.account.clone();
        user_repository::delete(&self.db, user)
            .await
            .map_err(|_| AppError::internal("failed to delete user"))?;

        audit_log_repository::append(
            &self.db,
            Some(actor_id),
            "DELETE_USER",
            "user",
            Some(id),
            Some(format!("deleted user {}", account)),
        )
        .await
        .map_err(|_| AppError::internal("failed to write audit log"))?;

        Ok(())
    }
}
