use najm_course_api::{Env, UsersItemDto, UsersItemDtoRaw};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let env = Env::new();
	let db = Surreal::new::<Ws>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

	let raw_users: Vec<UsersItemDtoRaw> = db.select("app_users").await?;
	let users: Vec<UsersItemDto> = raw_users.into_iter().map(Into::into).collect();

	for user in users {
		println!("{:?}", user);
	}

	Ok(())
}
