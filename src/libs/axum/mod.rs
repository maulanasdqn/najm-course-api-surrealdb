use crate::{redisdb_init, Env, RedisClient};
use crate::{surrealdb_init, SurrealClient};
use axum::{serve, Router};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: FnOnce(SurrealClient, RedisClient) -> Fut,
	Fut: Future<Output = Router>,
{
	let env = Env::new();
	let surrealdb = surrealdb_init().await.expect("Failed surrealdb");
	let redisdb = redisdb_init().await.expect("Failed redisdb");
	let router = router_fn(surrealdb, redisdb).await;
	let port = env.port;
	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("Listening on http://{}", addr);
	match serve(listener, router).await {
		Ok(_) => println!("Server stopped gracefully."),
		Err(err) => println!("Server encountered an error: {}", err),
	}
}
