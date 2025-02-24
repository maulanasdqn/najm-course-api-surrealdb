use super::auth_dto::AuthLoginRequestDto;
use crate::{success_response, AppState, ResponseSuccessDto};
use axum::response::Response;

const USERS_KEY: &str = "users";

pub async fn mutation_login(
	params: AuthLoginRequestDto,
	state: &AppState,
) -> Response {
	let _users: Option<AuthLoginRequestDto> = state
		.surrealdb
		.select((USERS_KEY, &*params.email))
		.await
		.unwrap();

	let mut redis_conn = state
		.redisdb
		.get_connection()
		.expect("failed to get connection");

	redis::cmd("SET")
		.arg("some_key")
		.arg("some_value")
		.query::<()>(&mut redis_conn)
		.expect("failed to set value");

	let response = ResponseSuccessDto {
		data: AuthLoginRequestDto {
			email: params.email,
			password: params.password,
		},
	};
	success_response(response)
}
