use axum::Router;

pub mod auth;
pub mod docs;
pub mod gacha;
pub mod users;

pub use auth::*;
pub use docs::*;
pub use gacha::*;
pub use users::*;

pub async fn routes() -> Router {
	Router::new()
		.nest("/auth", auth_router())
		.nest("/gacha", gacha_router())
}
