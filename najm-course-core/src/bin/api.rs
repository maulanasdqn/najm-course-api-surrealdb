use najm_course_apis::{apps, axum_init};

#[tokio::main]
async fn main() {
	env_logger::init();
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		apps(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
