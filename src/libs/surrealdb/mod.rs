use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Result, Surreal};

use crate::SurrealClient;

pub async fn surrealdb_init() -> Result<SurrealClient> {
	let db = Surreal::<Client>::init();
	db.connect::<Ws>("ws://localhost:8000").await?;
	db.use_ns("test").use_db("test").await?;
	Ok(db)
}
