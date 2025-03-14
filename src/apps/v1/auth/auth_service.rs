use axum::{http::StatusCode, response::Response};

use super::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthRegisterRequestDto,
	AuthRepository, TokenDto,
};
use crate::{
	common_response, encode_access_token, encode_refresh_token, hash_password,
	success_response, v1::UsersItemDto, verify_password, AppState,
	ResponseSuccessDto, TokenSub,
};

pub struct AuthService;

impl AuthService {
	pub async fn mutation_login(
		payload: AuthLoginRequestDto,
		state: &AppState,
	) -> Response {
		let repository = AuthRepository::new(state);
		match repository.query_user_by_email(payload.email.clone()).await {
			Ok(user) => {
				let is_password_correct =
					verify_password(&payload.password, &user.password)
						.unwrap_or(false);

				if is_password_correct {
					common_response(
						StatusCode::BAD_REQUEST,
						"Email or password not correct",
					);
				}

				let access_token = encode_access_token(TokenSub {
					email: payload.email.clone(),
					role_name: "Admin".to_string(),
				});

				let refresh_token = encode_refresh_token(TokenSub {
					email: payload.email.clone(),
					role_name: "Admin".to_string(),
				});

				let response = ResponseSuccessDto {
					data: AuthLoginResponsetDto {
						user: UsersItemDto {
							fullname: user.fullname.clone(),
							email: user.email.clone(),
						},
						token: TokenDto {
							access_token: access_token.unwrap(),
							refresh_token: refresh_token.unwrap(),
						},
					},
				};

				if !repository
					.query_store_user_data(AuthRegisterRequestDto {
						fullname: user.fullname,
						password: user.password,
						email: user.email,
					})
					.is_ok()
				{
					return common_response(
						StatusCode::BAD_REQUEST,
						"Failed to store data",
					);
				}

				success_response(response)
			}
			Err(err) => common_response(StatusCode::UNAUTHORIZED, &err.to_string()),
		}
	}

	pub async fn mutation_register(
		payload: AuthRegisterRequestDto,
		state: &AppState,
	) -> Response {
		let repository = AuthRepository::new(state);
		if repository
			.query_user_by_email(payload.email.clone())
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

		match repository.query_create_user(new_user).await {
			Ok(_) => common_response(StatusCode::CREATED, "Registration successful"),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}
}
