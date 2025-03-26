use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

use super::{RolesItemDto, RolesRequestCreateDto, RolesRequestUpdateDto};
use crate::{
	v1::roles_service::RolesService, AppState, MessageResponseDto, MetaRequestDto,
	ResponseListSuccessDto, ResponseSuccessDto,
};

#[utoipa::path(
    get,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/roles",
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
        (status = 200, description = "Get role list", body = ResponseListSuccessDto<Vec<RolesItemDto>>)
    ),
    tag = "Roles"
)]
pub async fn get_role_list(
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	RolesService::get_role_list(&state, meta).await
}

#[utoipa::path(
    get,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/roles/detail/{id}",
    params(("id" = String, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Get role by ID", body = ResponseSuccessDto<RolesItemDto>)
    ),
    tag = "Roles"
)]
pub async fn get_role_by_id(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	RolesService::get_role_by_id(&state, id).await
}

#[utoipa::path(
    post,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/roles/create",
    request_body = RolesRequestCreateDto,
    responses(
        (status = 201, description = "Create new role", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn post_create_role(
	Extension(state): Extension<AppState>,
	Json(payload): Json<RolesRequestCreateDto>,
) -> impl IntoResponse {
	RolesService::create_role(&state, payload).await
}

#[utoipa::path(
    put,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/roles/update/{id}",
    request_body = RolesRequestUpdateDto,
    responses(
        (status = 200, description = "Update role", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn put_update_role(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<RolesRequestUpdateDto>,
) -> impl IntoResponse {
	RolesService::update_role(&state, id, payload).await
}

#[utoipa::path(
    delete,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/roles/delete/{id}",
    responses(
        (status = 200, description = "Delete role", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn delete_role(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	RolesService::delete_role(&state, id).await
}
