use super::Env;
use crate::SurrealClient;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

pub mod resource;
pub use resource::*;

pub async fn surrealdb_init() -> Result<SurrealClient> {
	let env = Env::new();
	let db = Surreal::<Client>::init();
	db.connect::<Http>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;
	Ok(db)
}
