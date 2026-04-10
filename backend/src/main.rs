mod entity;
mod error;
mod middleware;
mod models;
mod repository;
mod routes;
mod service;
mod state;

use axum::{Router, middleware::from_fn_with_state, routing::{get, patch, post}};
use state::AppState;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "mini_kms_admin_backend=debug,tower_http=info".into()))
        .with(fmt::layer())
        .init();

    let state = AppState::new()
        .await
        .expect("failed to initialize application state");

    let protected_routes = Router::new()
        .route("/api/system/status", get(routes::system::get_status))
        .route("/api/users", get(routes::users::list_users).post(routes::users::create_user))
        .route("/api/users/{id}", patch(routes::users::update_user).delete(routes::users::delete_user))
        .route("/api/devices", get(routes::devices::list_devices))
        .route("/api/devices/{id}", get(routes::devices::get_device))
        .route("/api/keypacks", get(routes::keypacks::list_keypacks))
        .route("/api/audit-logs", get(routes::audit_logs::list_audit_logs))
        .layer(from_fn_with_state(state.clone(), middleware::auth::auth_middleware));

    let app = Router::new()
        .route("/api/login", post(routes::auth::login))
        .merge(protected_routes)
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind listener");

    tracing::info!("backend listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("server error");
}
