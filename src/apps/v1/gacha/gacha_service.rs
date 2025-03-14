use super::{GachaCreateItemRequestDto, GachaRepository, GachaRequestDto};
use crate::{common_response, AppState};
use axum::{http::StatusCode, response::Response};

pub struct GachaService;

impl GachaService {
	pub async fn mutation_create_gacha(
		payload: GachaRequestDto,
		state: &AppState,
	) -> Response {
		let repository = GachaRepository::new(state);

		match repository.query_create_gacha(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_create_gacha_item(
		payload: GachaCreateItemRequestDto,
		state: &AppState,
	) -> Response {
		let repository = GachaRepository::new(state);

		match repository.query_create_gacha_item(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}
}
