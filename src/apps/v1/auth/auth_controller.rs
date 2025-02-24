use axum::{response::IntoResponse, Extension, Json};

use super::{mutation_login, AuthLoginRequestDto};
use crate::AppState;

pub async fn post_login(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	mutation_login(payload, &state).await
}
