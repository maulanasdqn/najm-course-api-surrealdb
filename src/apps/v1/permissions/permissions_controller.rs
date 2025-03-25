use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

use crate::{
	v1::{
		permissions_dto::{PermissionsItemDto, PermissionsRequestDto},
		permissions_service::PermissionsService,
	},
	AppState, MessageResponseDto, MetaRequestDto, ResponseListSuccessDto,
	ResponseSuccessDto,
};

#[utoipa::path(
	get,
	path = "/v1/permissions",
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
		(status = 200, description = "Get permission list", body = ResponseListSuccessDto<Vec<PermissionsItemDto>>)
	),
	tag = "Permissions"
)]
pub async fn get_permission_list(
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	PermissionsService::get_permission_list(&state, meta).await
}

#[utoipa::path(
	get,
	path = "/v1/permissions/detail/{id}",
	params(("id" = String, Path, description = "Permission ID")),
	responses(
		(status = 200, description = "Get permission by ID", body = ResponseSuccessDto<PermissionsItemDto>)
	),
	tag = "Permissions"
)]
pub async fn get_permission_by_id(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	PermissionsService::get_permission_by_id(&state, id).await
}

#[utoipa::path(
	post,
	path = "/v1/permissions/create",
	request_body = PermissionsRequestDto,
	responses(
		(status = 201, description = "Create new permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn post_create_permission(
	Extension(state): Extension<AppState>,
	Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
	PermissionsService::create_role(&state, payload).await
}

#[utoipa::path(
	put,
	path = "/v1/permissions/update/{id}",
	request_body = PermissionsRequestDto,
	responses(
		(status = 200, description = "Update permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn put_update_permission(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
	PermissionsService::update_permission(&state, payload, id).await
}

#[utoipa::path(
	delete,
	path = "/v1/permissions/delete/{id}",
	responses(
		(status = 200, description = "Delete permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn delete_permission(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	PermissionsService::delete_permission(&state, id).await
}
