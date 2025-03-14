use crate::{common_response, decode_access_token, AppState};
use axum::{
	extract::Request,
	http::{header::AUTHORIZATION, StatusCode},
	middleware::Next,
	response::Response,
	Extension,
};
use std::convert::Infallible;

use super::{AuthQueryByEmailResponse, AuthRepository};

pub async fn auth_middleware(
	Extension(state): Extension<AppState>,
	mut req: Request,
	next: Next,
) -> Result<Response, Infallible> {
	let auth_header = match req.headers().get(AUTHORIZATION) {
		Some(h) => h.to_str().unwrap_or_default(),
		None => {
			return Ok(common_response(
				StatusCode::UNAUTHORIZED,
				"You are not authorized",
			));
		}
	};

	let token = auth_header.strip_prefix("Bearer ").unwrap_or("");

	let token_data = match decode_access_token(token) {
		Ok(data) => data,
		Err(_) => {
			return Ok(common_response(
				StatusCode::UNAUTHORIZED,
				&format!("Invalid or expired token"),
			));
		}
	};

	let repository = AuthRepository::new(&state);

	let user: Option<AuthQueryByEmailResponse> = match repository
		.query_user_by_email(token_data.claims.sub.clone())
		.await
	{
		Ok(user) => Some(user),
		Err(err) => {
			return Ok(common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&format!("DB error: {}", err),
			))
		}
	};

	if user.is_none() {
		return Ok(common_response(
			StatusCode::UNAUTHORIZED,
			"Unauthorized user",
		));
	}

	req.extensions_mut().insert(user.unwrap());

	Ok(next.run(req).await)
}
