use axum::{routing::get, Router};
use v1::auth_router;

pub mod v1;
pub mod v2;

pub async fn apps() -> Router {
	let v1_routes = Router::new()
		.nest("/auth", auth_router())
		.route("/", get(|| async { "Comming Soon v1" }));

	let v2_routes = Router::new().route("/", get(|| async { "Comming Soon v2" }));

	Router::new().nest("/v1", v1_routes).nest("/v2", v2_routes)
}
