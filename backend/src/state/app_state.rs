use sea_orm::Database;

use crate::{
    service::{
        audit_service::AuditService, auth_service::AuthService, device_service::DeviceService,
        keypack_service::KeypackService, system_service::SystemService, user_service::UserService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub device_service: DeviceService,
    pub keypack_service: KeypackService,
    pub system_service: SystemService,
    pub audit_service: AuditService,
}

impl AppState {
    pub async fn new() -> Result<Self, sea_orm::DbErr> {
        let jwt_secret = "mini-kms-admin-dev-secret".to_string();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@127.0.0.1:3306/mini_kms_admin".to_string());
        let db = Database::connect(&database_url).await?;

        Ok(Self {
            auth_service: AuthService::new(db.clone(), jwt_secret),
            user_service: UserService::new(db.clone()),
            device_service: DeviceService::new(db.clone()),
            keypack_service: KeypackService::new(db.clone()),
            system_service: SystemService::new(db.clone()),
            audit_service: AuditService::new(db),
        })
    }
}
