use super::{OptionsCreateRequestDto, OptionsRepository, OptionsUpdateRequestDto};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};

pub struct OptionsService;

impl OptionsService {
	pub async fn get_option_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = OptionsRepository::new(state);
		match repo.query_option_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_option_by_id(state: &AppState, id: String) -> Response {
		let repo = OptionsRepository::new(state);
		match repo.query_option_by_id(id).await {
			Ok(option) => success_response(ResponseSuccessDto { data: option }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_option(
		state: &AppState,
		payload: OptionsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = OptionsRepository::new(state);
		match repo.query_create_option(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_option(
		state: &AppState,
		id: String,
		payload: OptionsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = OptionsRepository::new(state);
		match repo.query_update_option(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_option(state: &AppState, id: String) -> Response {
		let repo = OptionsRepository::new(state);
		match repo.query_raw_option_by_id(&id).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Option not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete_option(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
