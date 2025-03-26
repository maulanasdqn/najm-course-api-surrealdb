use najm_course_api::{apps, axum_init};

#[tokio::main]
async fn main() {
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		apps(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
