use imphnen_cms_be::{apps, libs::axum_init};

#[tokio::main]
async fn main() {
	axum_init(apps).await;
}
