use axum::{serve, Router};
use std::env;
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: Fn() -> Fut,
	Fut: Future<Output = Router>,
{
	let router = router_fn().await;
	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		env::var("PORT")
			.unwrap_or("3000".to_string())
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
