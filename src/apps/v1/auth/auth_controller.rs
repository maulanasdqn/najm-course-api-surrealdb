use super::{mutation_login, AuthLoginRequestDto};
use axum::{response::Response, Json};

pub async fn post_login(Json(payload): Json<AuthLoginRequestDto>) -> Response {
	mutation_login(payload).await
}
