use axum_test::{TestResponse, TestServer};
use najm_course_entities::AppState;
use surrealdb::{
	Surreal,
	engine::{local::Mem, remote::ws::Ws},
	opt::auth::Root,
};

pub async fn create_mock_app_state() -> AppState {
	let db_mem = Surreal::new::<Mem>(()).await.unwrap();
	let db_ws = Surreal::new::<Ws>("localhost:8000").await.unwrap();
	db_mem.use_ns("test").use_db("test").await.unwrap();
	db_ws
		.signin(Root {
			username: "root",
			password: "root",
		})
		.await
		.unwrap();
	db_ws.use_ns("test").use_db("test").await.unwrap();
	AppState {
		surrealdb_mem: db_mem,
		surrealdb_ws: db_ws,
	}
}

pub async fn cleanup_db() {
	let app_state = create_mock_app_state().await;
	let _ = app_state
		.surrealdb_mem
		.query(
			r#"
    REMOVE TABLE app_users;
    REMOVE TABLE app_roles;
    REMOVE TABLE app_users_cache;
    REMOVE TABLE app_otp_cache;
"#,
		)
		.await;
	let _ = app_state
		.surrealdb_ws
		.query(
			r#"
    REMOVE TABLE app_users;
    REMOVE TABLE app_roles;
    REMOVE TABLE app_users_cache;
    REMOVE TABLE app_otp_cache;
"#,
		)
		.await;
}

pub fn test_auth_token_with_permissions(perms: Vec<&str>) -> String {
	let permissions_str = perms.join(",");
	format!("Bearer test-token:{}", permissions_str)
}

pub async fn authorized(
	server: &TestServer,
	method: &str,
	path: &str,
	permissions: Vec<&str>,
) -> TestResponse {
	let token = test_auth_token_with_permissions(permissions);
	match method {
		"GET" => server.get(path).add_header("Authorization", &token).await,
		"POST" => server.post(path).add_header("Authorization", &token).await,
		"PUT" => server.put(path).add_header("Authorization", &token).await,
		"DELETE" => {
			server
				.delete(path)
				.add_header("Authorization", &token)
				.await
		}
		_ => panic!("Unsupported HTTP method"),
	}
}
