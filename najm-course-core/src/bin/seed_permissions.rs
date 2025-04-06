use najm_course_apis::{get_iso_date, Env};
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
	let permissions = vec![
		(
			"023e2dfe-93c3-4008-94a8-b5dff403f73b",
			"Create Users",
			Some("2025-01-29T06:08:23.838311+00"),
			Some("2025-01-29T06:08:23.838312+00"),
		),
		(
			"0269ed71-0ae0-4c43-ad29-e3d861d8f9a0",
			"Create Permissions",
			None,
			None,
		),
		(
			"299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b",
			"Update Permissions",
			Some("2025-01-29T05:11:01.265+00"),
			Some("2025-01-29T05:11:01.265001+00"),
		),
		(
			"319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
			"Create Roles",
			None,
			None,
		),
		(
			"319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
			"Read Detail Users",
			None,
			None,
		),
		(
			"35b0d992-65c8-4b62-b030-e6e0320e4048",
			"Delete Roles",
			None,
			None,
		),
		(
			"4da8b434-89f9-4d91-85ae-eebd63cdbeda",
			"Update Activate Users",
			Some("2025-02-01T12:38:09.741726+00"),
			Some("2025-02-01T12:38:09.741727+00"),
		),
		(
			"529fe4da-7e20-4c76-8bc1-d7f7c121218f",
			"Create Tests",
			Some("2025-02-24T16:52:27.316909+00"),
			Some("2025-02-24T16:52:27.316918+00"),
		),
		(
			"73888d18-b3e9-4f62-95a5-ba2c0d69fccb",
			"Read Detail Roles",
			Some("2025-01-29T05:13:06.445925+00"),
			Some("2025-01-29T10:31:46.408564+00"),
		),
		(
			"79dfe6dc-748c-4b9c-9535-b896b391d676",
			"Delete Tests",
			Some("2025-02-24T16:52:27.455878+00"),
			Some("2025-02-24T16:52:27.455888+00"),
		),
		(
			"7c15e31d-36e2-49f9-97db-138c03fb0cf6",
			"Read List Users",
			Some("2025-01-28T15:02:41.772931+00"),
			Some("2025-01-28T15:02:41.772933+00"),
		),
		(
			"7d4b1379-4960-416a-b045-98cd82c0cac9",
			"Read Detail Sessions",
			Some("2025-02-24T16:52:26.886664+00"),
			Some("2025-02-24T16:52:26.886673+00"),
		),
		(
			"811d386b-e5f0-4e00-a164-f3d885197e30",
			"Update Tests",
			Some("2025-02-24T16:52:27.385216+00"),
			Some("2025-02-24T16:52:27.385225+00"),
		),
		(
			"8195eeb8-e64f-4172-aa57-596492c84a72",
			"Read List Permissions",
			Some("2025-01-28T15:05:28.6299+00"),
			Some("2025-01-28T15:05:28.629901+00"),
		),
		(
			"81eba91d-b8ab-44b9-bbfe-4e6da2f98952",
			"Read List Tests",
			Some("2025-02-24T16:52:27.179542+00"),
			Some("2025-02-24T16:52:27.179551+00"),
		),
		(
			"8cfd3b4d-0a41-456d-88e5-6c21cef1766a",
			"Delete Sessions",
			Some("2025-02-24T16:52:27.111123+00"),
			Some("2025-02-24T16:52:27.111132+00"),
		),
		(
			"9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
			"Read List Roles",
			None,
			None,
		),
		(
			"96df0689-2ae9-4894-bf00-837c19415e5c",
			"Delete Users",
			Some("2025-02-02T06:52:05.195565+00"),
			Some("2025-02-02T06:52:05.195565+00"),
		),
		(
			"98b3dc4c-0124-461f-afcd-166637c5e6e8",
			"Update Users",
			Some("2025-01-29T05:34:40.621554+00"),
			Some("2025-01-29T05:34:40.621555+00"),
		),
		(
			"a00d5608-4c48-4542-845c-dfe004687022",
			"Update Roles",
			None,
			None,
		),
		(
			"b2dc3928-86ba-4c59-a03d-0b57d5183ebc",
			"Delete Permissions",
			Some("2025-01-29T05:14:22.511084+00"),
			Some("2025-01-29T05:14:22.511085+00"),
		),
		(
			"b70733db-b1c8-4aa3-a10f-b7cf773d896b",
			"Create Sessions",
			Some("2025-02-24T16:52:26.974279+00"),
			Some("2025-02-24T16:52:26.97429+00"),
		),
		(
			"c0a31b2c-3f0c-4e82-b018-e60ba8674112",
			"Update Sessions",
			Some("2025-02-24T16:52:27.042858+00"),
			Some("2025-02-24T16:52:27.042866+00"),
		),
		(
			"cab6aff5-e9c6-4ed3-afe9-93ef927e1f92",
			"Read List Sessions",
			Some("2025-02-22T15:38:09.521014+00"),
			Some("2025-02-22T15:38:25.964821+00"),
		),
		(
			"dad435cf-042c-41bd-a946-cea61ed2ffbc",
			"Read Detail Permissions",
			Some("2025-01-28T15:07:10.990214+00"),
			Some("2025-01-28T15:07:10.990214+00"),
		),
		(
			"f768aff5-8011-4439-b901-d8793c60d841",
			"Read Detail Tests",
			Some("2025-02-24T16:52:27.2483+00"),
			Some("2025-02-24T16:52:27.248308+00"),
		),
	];
	for (id, name, _created_at, _updated_at) in permissions {
		db.query("CREATE type::thing('app_permissions', $id) CONTENT $data")
			.bind(("id", id))
			.bind((
				"data",
				json!({
						"name": name,
						"is_deleted": false,
						"created_at": get_iso_date(),
						"updated_at": get_iso_date()
				}),
			))
			.await?;
		println!("✅ Inserted: {}", name);
	}
	println!("✅ Semua permissions berhasil disimpan ke SurrealDB!");
	Ok(())
}
