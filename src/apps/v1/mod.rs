use axum::Router;

pub mod auth;
pub mod users;

pub use auth::*;
pub use users::*;

pub async fn routes() -> Router {
	Router::new().nest("/auth", auth::auth_router())
}
