use super::auth_dto::AuthLoginRequestDto;
use crate::{success_response, ResponseSuccessDto};
use axum::response::Response;

pub async fn mutation_login(params: AuthLoginRequestDto) -> Response {
	let response = ResponseSuccessDto {
		data: AuthLoginRequestDto {
			email: params.email,
			password: params.password,
		},
	};
	success_response(response)
}
