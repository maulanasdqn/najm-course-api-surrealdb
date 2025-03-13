use super::{
	mutation_login, mutation_register, AuthLoginRequestDto, AuthRegisterRequestDto,
};
use crate::AppState;
use axum::{response::IntoResponse, Extension, Json};

pub async fn post_login(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	mutation_login(payload, &state).await
}

pub async fn post_register(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthRegisterRequestDto>,
) -> impl IntoResponse {
	mutation_register(payload, &state).await
}
