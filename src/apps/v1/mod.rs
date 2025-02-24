use axum::Router;

pub mod auth;

pub async fn routes() -> Router {
	Router::new().nest("/auth", auth::auth_router())
}
