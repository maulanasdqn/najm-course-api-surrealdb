use crate::{AppState, RedisClient, SurrealClient};
use axum::{Extension, Router};

pub mod v1;
pub mod v2;

pub async fn apps(surrealdb: SurrealClient, redisdb: RedisClient) -> Router {
	let state = AppState { surrealdb, redisdb };
	Router::new()
		.nest("/v1", v1::routes().await)
		.nest("/v2", v2::routes().await)
		.layer(Extension(state))
}
