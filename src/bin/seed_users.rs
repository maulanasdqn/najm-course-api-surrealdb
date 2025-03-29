use najm_course_api::{get_iso_date, make_thing, Env};
use serde_json::json;
use std::error::Error;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
	let users = vec![
		(
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2",
			"admin@example.com",
			"Admin",
			"f6b03f25-e416-4893-ac88-caaa690afb07",
		),
		(
			"a4d23fb5-9e31-423c-9842-fbd6e75a5298",
			"staff@example.com",
			"Staff",
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
		),
		(
			"d5e89c12-72af-4b1a-abc3-ff1234567890",
			"student@example.com",
			"Student",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
	];
	for (id, email, name, role_id) in users {
		db.query("CREATE type::thing('app_users', $id) CONTENT $data")
			.bind(("id", id))
			.bind((
				"data",
				json!({
						"email": email,
						"name": name,
						"phone_number": "081234567890",
						"student_type": "TNI",
						"role": make_thing("app_roles", role_id),
						"is_deleted": false,
						"created_at": get_iso_date(),
						"updated_at": get_iso_date(),
				}),
			))
			.await?;

		println!("✅ Inserted user: {} ({})", name, email);
	}
	println!("✅ Semua users berhasil disimpan ke SurrealDB!");
	Ok(())
}
