use crate::{AppState, RedisClient, SurrealClient};
use axum::{response::Redirect, routing::get, Extension, Router};
use utoipa_swagger_ui::SwaggerUi;

pub mod v1;
pub mod v2;

pub async fn apps(surrealdb: SurrealClient, redisdb: RedisClient) -> Router {
	let state = AppState { surrealdb, redisdb };
	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", v1::routes().await)
		.nest("/v2", v2::routes().await)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", v1::docs_router()))
		.layer(Extension(state))
}
