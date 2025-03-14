use super::{AuthLoginRequestDto, AuthRegisterRequestDto, AuthService};
use crate::{v1::AuthLoginResponsetDto, AppState};
use axum::{response::IntoResponse, Extension, Json};

use crate::{MessageResponseDto, ResponseSuccessDto};

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "Login successful", body = ResponseSuccessDto<AuthLoginResponsetDto>),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_login(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_login(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    request_body = AuthRegisterRequestDto,
    responses(
        (status = 200, description = "Login successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_register(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthRegisterRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_register(payload, &state).await
}
