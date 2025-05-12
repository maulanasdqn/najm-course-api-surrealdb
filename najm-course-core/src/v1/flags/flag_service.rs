use super::{FlagsItemDto, FlagsRepository, FlagsRequestDto, FlagsSchema};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::http::StatusCode;
use axum::response::Response;

pub struct FlagsService;

impl FlagsService {
	pub async fn get_flag_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = FlagsRepository::new(state);
		match repo.query_flag_list(meta).await {
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

	pub async fn get_flag_by_id(state: &AppState, id: String) -> Response {
		let repo = FlagsRepository::new(state);
		match repo.query_flag_by_id(id).await {
			Ok(flag_raw) => {
				let flag: FlagsItemDto = flag_raw.from();
				success_response(ResponseSuccessDto { data: flag })
			}
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_flag(state: &AppState, payload: FlagsRequestDto) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = FlagsRepository::new(state);
		match repo.query_flag_by_name(payload.name.clone()).await {
			Ok(_role) => {
				return common_response(StatusCode::CONFLICT, "Flag name already exists");
			}
			Err(err) if err.to_string().contains("not found") => {}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_create_flag(FlagsSchema::new(payload)).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_flag(
		state: &AppState,
		payload: FlagsRequestDto,
		id: String,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = FlagsRepository::new(state);
		match repo
			.query_update_flag(FlagsSchema::update(&id, payload))
			.await
		{
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Flag not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}

	pub async fn delete_flag(state: &AppState, id: String) -> Response {
		let repo = FlagsRepository::new(state);
		match repo.query_delete_flag(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Flag not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}
}
