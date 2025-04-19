use crate::{MinioClient, StorageState};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn storage_state() -> Result<StorageState, Box<dyn std::error::Error>> {
	let minio_client = MinioClient::new().await?;
	Ok(StorageState {
		minio: Arc::new(Mutex::new(minio_client)),
	})
}
