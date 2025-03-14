use crate::SurrealClient;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::{Result, Surreal};

pub mod resource;
pub use resource::*;

pub async fn surrealdb_init() -> Result<SurrealClient> {
	let db = Surreal::<Client>::init();
	db.connect::<Http>("localhost:8000").await?;
	db.signin(surrealdb::opt::auth::Root {
		username: "root",
		password: "root",
	})
	.await?;
	db.use_ns("test").use_db("test").await?;
	Ok(db)
}
