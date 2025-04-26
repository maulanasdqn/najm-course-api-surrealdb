use answers::answers_router;
use axum::{middleware::from_fn, Router};
pub mod answers;
pub mod auth;
pub mod docs;
pub mod options;
pub mod permissions;
pub mod questions;
pub mod roles;
pub mod sessions;
pub mod storage;
pub mod tests;
pub mod users;

pub use auth::*;
pub use docs::*;
pub use options::*;
pub use permissions::*;
pub use questions::*;
pub use roles::*;
pub use storage::*;
pub use tests::*;
pub use users::*;

pub async fn routes() -> Router {
	let public_routes = Router::new().nest("/auth", auth_router());
	let protected_routes = Router::new()
		.nest("/users", users_router())
		.nest("/roles", roles_router())
		.nest("/sessions", sessions_router())
		.nest("/permissions", permissions_router())
		.nest("/options", options_router())
		.nest("/questions", questions_router())
		.nest("/tests", tests_router())
		.nest("/answers", answers_router())
		.nest("/storage", storage_router().await)
		.layer(from_fn(auth_middleware::auth_middleware));
	Router::new().merge(public_routes).merge(protected_routes)
}
