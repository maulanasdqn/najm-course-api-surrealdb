use super::{TestsCreateRequestDto, TestsRepository, TestsUpdateRequestDto};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};
use validator::Validate;

pub struct TestsService;

impl TestsService {
	pub async fn get_test_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = TestsRepository::new(state);
		match repo.query_test_list(meta).await {
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

	pub async fn get_test_by_id(state: &AppState, id: String) -> Response {
		let repo = TestsRepository::new(state);
		match repo.query_test_by_id(&id).await {
			Ok(test) => success_response(ResponseSuccessDto { data: test }),
			Err(e) => {
				let status = if e.to_string().contains("not found") {
					StatusCode::NOT_FOUND
				} else {
					StatusCode::INTERNAL_SERVER_ERROR
				};
				common_response(status, &e.to_string())
			}
		}
	}

	pub async fn create_test(
		state: &AppState,
		payload: TestsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		if payload.questions.is_empty() {
			return common_response(
				StatusCode::BAD_REQUEST,
				"At least one question is required",
			);
		}
		for question in &payload.questions {
			if let Err(e) = question.validate() {
				return common_response(StatusCode::BAD_REQUEST, &e.to_string());
			}
			for option in &question.options {
				if let Err(e) = option.validate() {
					return common_response(StatusCode::BAD_REQUEST, &e.to_string());
				}
			}
		}
		let repo = TestsRepository::new(state);
		match repo.query_create_test(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_test(
		state: &AppState,
		id: String,
		payload: TestsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		for question in &payload.questions {
			if let Err(e) = question.validate() {
				return common_response(StatusCode::BAD_REQUEST, &e.to_string());
			}
			for option in &question.options {
				if let Err(e) = option.validate() {
					return common_response(StatusCode::BAD_REQUEST, &e.to_string());
				}
			}
		}
		let repo = TestsRepository::new(state);
		match repo.query_update_test(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				let msg = e.to_string();
				let status = if msg.contains("not found") {
					StatusCode::NOT_FOUND
				} else if msg.contains("deleted") || msg.contains("must") {
					StatusCode::BAD_REQUEST
				} else {
					StatusCode::INTERNAL_SERVER_ERROR
				};
				common_response(status, &msg)
			}
		}
	}

	pub async fn delete_test(state: &AppState, id: String) -> Response {
		let repo = TestsRepository::new(state);
		match repo.query_delete_test(id.clone()).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				let err_msg = e.to_string();
				if err_msg.contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Test not found")
				} else if err_msg.contains("already deleted") {
					common_response(StatusCode::BAD_REQUEST, "Test already deleted")
				} else {
					common_response(StatusCode::INTERNAL_SERVER_ERROR, &err_msg)
				}
			}
		}
	}
}
