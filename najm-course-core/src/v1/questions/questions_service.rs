use super::{
	QuestionsCreateRequestDto, QuestionsRepository, QuestionsUpdateRequestDto,
};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};

pub struct QuestionsService;

impl QuestionsService {
	pub async fn get_question_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = QuestionsRepository::new(state);
		match repo.query_question_list(meta).await {
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

	pub async fn get_question_by_id(state: &AppState, id: String) -> Response {
		let repo = QuestionsRepository::new(state);
		match repo.query_question_by_id(&id.clone()).await {
			Ok(question) => success_response(ResponseSuccessDto { data: question }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_question(
		state: &AppState,
		payload: QuestionsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = QuestionsRepository::new(state);
		match repo.query_create_question(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => {
				let msg = e.to_string();
				if msg.contains("must not be empty") || msg.contains("non-empty label") {
					common_response(StatusCode::BAD_REQUEST, &msg)
				} else {
					common_response(StatusCode::INTERNAL_SERVER_ERROR, &msg)
				}
			}
		}
	}

	pub async fn update_question(
		state: &AppState,
		id: String,
		payload: QuestionsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = QuestionsRepository::new(state);
		match repo.query_update_question(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_question(state: &AppState, id: String) -> Response {
		let repo = QuestionsRepository::new(state);
		match repo.query_raw_question_by_id(&id).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Question not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete_question(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
