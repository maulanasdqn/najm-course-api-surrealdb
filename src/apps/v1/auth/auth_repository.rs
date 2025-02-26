use super::{auth_dto::AuthLoginRequestDto, AuthRegisterRequestDto};
use crate::{
	common_response, hash_password, success_response, v1::UsersItemDto, AppState,
	ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};
use serde_json;

const USERS_KEY: &str = "users";

pub async fn mutation_login(
	params: AuthLoginRequestDto,
	state: &AppState,
) -> Response {
	let user: Option<AuthLoginRequestDto> = match state
		.surrealdb
		.select((USERS_KEY, params.email.as_str()))
		.await
	{
		Ok(user) => user,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			);
		}
	};

	let mut redis_conn = match state.redisdb.get_connection() {
		Ok(conn) => conn,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			);
		}
	};

	let user_json = match serde_json::to_string(&user) {
		Ok(json) => json,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			);
		}
	};

	if let Err(err) = redis::cmd("SET")
		.arg("users_data")
		.arg(user_json)
		.query::<()>(&mut redis_conn)
	{
		return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
	}

	let response = ResponseSuccessDto { data: params };

	success_response(response)
}

pub async fn mutation_register(
	params: AuthRegisterRequestDto,
	state: &AppState,
) -> Response {
	let user_key = format!("{}:{}", USERS_KEY, params.email);

	let existing_user: Option<UsersItemDto> =
		match state.surrealdb.select(&user_key).await {
			Ok(user) => user,
			Err(err) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}
		};

	if existing_user.is_some() {
		return common_response(StatusCode::CONFLICT, "User already exists");
	}

	let hashed_password = hash_password(&params.password);

	let created_user: AuthLoginRequestDto =
		match state.surrealdb.create(&user_key, &params).await {
			Ok(user) => Some(user),
			Err(err) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}
		};

	let user_json = match serde_json::to_string(&created_user) {
		Ok(json) => json,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			);
		}
	};

	common_response(StatusCode::CREATED, "Success Register User")
}
