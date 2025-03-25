use crate::{
	common_response, success_list_response, success_response, AppState,
	MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::http::StatusCode;

use super::{RolesRepository, RolesRequestCreateDto, RolesRequestUpdateDto};

pub struct RolesService;

impl RolesService {
	pub async fn get_role_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> axum::response::Response {
		let repo = RolesRepository::new(state);
		match repo.query_role_list(meta).await {
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

	pub async fn get_role_by_id(
		state: &AppState,
		id: String,
	) -> axum::response::Response {
		let repo = RolesRepository::new(state);
		match repo.query_role_by_id(id).await {
			Ok(role) => success_response(ResponseSuccessDto { data: role }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_role(
		state: &AppState,
		payload: RolesRequestCreateDto,
	) -> axum::response::Response {
		let repo = RolesRepository::new(state);
		match repo.query_create_role(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_role(
		state: &AppState,
		id: String,
		payload: RolesRequestUpdateDto,
	) -> axum::response::Response {
		let repo = RolesRepository::new(state);
		match repo.query_update_role(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_role(
		state: &AppState,
		id: String,
	) -> axum::response::Response {
		let repo = RolesRepository::new(state);
		match repo.query_delete_role(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
