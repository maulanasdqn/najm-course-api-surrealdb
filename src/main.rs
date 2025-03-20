use axum_surrealdb_boilerplate::{apps, axum_init};

#[tokio::main]
async fn main() {
	axum_init(|db, redis| async { apps(db, redis).await }).await;
}
