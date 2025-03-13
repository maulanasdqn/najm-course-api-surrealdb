use axum::{http::StatusCode, response::Response};

use super::{
	query_create_user, query_user_by_email, AuthLoginRequestDto,
	AuthRegisterRequestDto,
};
use crate::{common_response, hash_password, verify_password, AppState};

pub async fn mutation_login(
	payload: AuthLoginRequestDto,
	state: &AppState,
) -> Response {
	match query_user_by_email(payload.email, state).await {
		Ok(user) => {
			let is_password_correct =
				verify_password(&payload.password, &user.password).unwrap_or(false);

			if is_password_correct {
				common_response(
					StatusCode::BAD_REQUEST,
					"Email or password not correct",
				);
			}

			common_response(StatusCode::OK, "Success Login")
		}
		Err(err) => common_response(StatusCode::UNAUTHORIZED, &err.to_string()),
	}
}

pub async fn mutation_register(
	payload: AuthRegisterRequestDto,
	state: &AppState,
) -> Response {
	if query_user_by_email(payload.email.clone(), state)
		.await
		.is_ok()
	{
		return common_response(StatusCode::BAD_REQUEST, "User already exists");
	}

	let hashed_password = match hash_password(&payload.password) {
		Ok(hash) => hash,
		Err(_) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to hash password",
			);
		}
	};

	let new_user = AuthRegisterRequestDto {
		email: payload.email,
		password: hashed_password,
		fullname: payload.fullname,
	};

	match query_create_user(new_user, state).await {
		Ok(_) => common_response(StatusCode::CREATED, "Registration successful"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}
