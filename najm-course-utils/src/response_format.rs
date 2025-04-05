use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use najm_course_entities::{ResponseListSuccessDto, ResponseSuccessDto};
use serde::Serialize;
use serde_json::json;

pub fn success_response<T: Serialize>(params: ResponseSuccessDto<T>) -> Response {
	(
		StatusCode::OK,
		Json(json!({
			"data": params.data,
			"version": "0.1.0",
		})),
	)
		.into_response()
}

pub fn success_list_response<T: Serialize>(
	params: ResponseListSuccessDto<T>,
) -> Response {
	(
		StatusCode::OK,
		Json(json!({
			"data": params.data,
			"meta": params.meta,
			"version": "0.1.0",
		})),
	)
		.into_response()
}

pub fn common_response(status: StatusCode, message: &str) -> Response {
	(
		status,
		Json(json!({
			"message": message,
			"version": "0.1.0",
		})),
	)
		.into_response()
}
