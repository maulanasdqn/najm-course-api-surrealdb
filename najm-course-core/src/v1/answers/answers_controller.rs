use super::{
	AnswersCreateRequestDto, AnswersResponseDto, AnswersService,
	AnswersUpdateRequestDto,
};
use crate::{
	permissions_guard, AppState, MessageResponseDto, MetaRequestDto, PermissionsEnum,
	ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers",
	params(
		("page" = Option<i64>, Query, description = "Page number"),
		("per_page" = Option<i64>, Query, description = "Items per page"),
		("search" = Option<String>, Query, description = "Search keyword"),
		("sort_by" = Option<String>, Query, description = "Sort by field"),
		("order" = Option<String>, Query, description = "Order ASC or DESC"),
		("filter" = Option<String>, Query, description = "Filter value"),
		("filter_by" = Option<String>, Query, description = "Field to filter by"),
	),
	responses(
		(status = 200, description = "Get answer list", body = ResponseListSuccessDto<Vec<AnswersResponseDto>>)
	),
	tag = "Answers"
)]
pub async fn get_answer_list(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::get_answer_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/detail/{id}",
	params(("id" = String, Path, description = "Answer ID")),
	responses(
		(status = 200, description = "Get answer by ID", body = ResponseSuccessDto<AnswersResponseDto>)
	),
	tag = "Answers"
)]
pub async fn get_answer_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::get_answer_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/create",
	request_body = AnswersCreateRequestDto,
	responses(
		(status = 201, description = "Create new answer", body = MessageResponseDto)
	),
	tag = "Answers"
)]
pub async fn post_create_answer(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<AnswersCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::create_answer(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/update/{id}",
	request_body = AnswersUpdateRequestDto,
	responses(
		(status = 200, description = "Update answer", body = MessageResponseDto)
	),
	tag = "Answers"
)]
pub async fn put_update_answer(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<AnswersUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::update_answer(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/delete/{id}",
	responses(
		(status = 200, description = "Delete answer", body = MessageResponseDto)
	),
	tag = "Answers"
)]
pub async fn delete_answer(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::delete_answer(&state, id).await,
		Err(response) => response,
	}
}
