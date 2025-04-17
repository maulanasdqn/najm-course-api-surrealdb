use super::{AnswersCreateRequestDto, AnswersRepository, AnswersUpdateRequestDto};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};

pub struct AnswersService;

impl AnswersService {
	pub async fn get_answer_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_list(meta).await {
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

	pub async fn get_answer_by_id(state: &AppState, id: String) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_by_id(&id).await {
			Ok(answer) => success_response(ResponseSuccessDto { data: answer }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_answer(
		state: &AppState,
		payload: AnswersCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = AnswersRepository::new(state);
		match repo.query_create(payload).await {
			Ok(id) => common_response(StatusCode::CREATED, &id),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_answer(
		state: &AppState,
		id: String,
		payload: AnswersUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = AnswersRepository::new(state);
		match repo.query_update(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_answer(state: &AppState, id: String) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_by_id(&id).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Answer not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
