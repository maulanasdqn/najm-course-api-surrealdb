use axum::{serve, Router};
use std::env;
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

use crate::{redisdb_init, RedisClient};
use crate::{surrealdb_init, SurrealClient};

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: FnOnce(SurrealClient, RedisClient) -> Fut,
	Fut: Future<Output = Router>,
{
	let surrealdb = surrealdb_init().await.expect("Failed surrealdb");
	let redisdb = redisdb_init().await.expect("Failed  redisdb");
	let router = router_fn(surrealdb, redisdb).await;

	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		env::var("PORT")
			.unwrap_or_else(|_| "3000".to_string())
			.parse()
			.unwrap(),
	));

	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("Listening on http://{}", addr);

	match serve(listener, router).await {
		Ok(_) => println!("Server stopped gracefully."),
		Err(err) => println!("Server encountered an error: {}", err),
	}
}
