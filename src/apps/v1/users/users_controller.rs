use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use crate::{v1::users_service::UsersService, AppState, MetaRequestDto};
use crate::{
	MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
	UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersDetailItemDto,
};

use super::{UsersListItemDto, UsersUpdateRequestDto};

#[utoipa::path(
	get,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users",
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
		(status = 200, description = "Get user list", body = ResponseListSuccessDto<Vec<UsersListItemDto>>)
	),
	tag = "Users"
)]
pub async fn get_user_list(
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	UsersService::get_user_list(&state, meta).await
}

#[utoipa::path(
	get,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/detail/{id}",
	params(
		("id" = String, Path, description = "User ID")
	),
	responses(
		(status = 200, description = "Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
	),
	tag = "Users"
)]
pub async fn get_user_by_id(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	UsersService::get_user_by_id(&state, id).await
}

#[utoipa::path(
	get,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/me",
	responses(
		(status = 200, description = "Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
	),
	tag = "Users"
)]
pub async fn get_user_me(
	Extension(state): Extension<AppState>,
	header: HeaderMap,
) -> impl IntoResponse {
	UsersService::get_user_me(header, &state).await
}

#[utoipa::path(
	post,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/create",
	request_body = UsersCreateRequestDto,
	responses(
		(status = 201, description = "Create new user", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn post_create_user(
	Extension(state): Extension<AppState>,
	Json(payload): Json<UsersCreateRequestDto>,
) -> impl IntoResponse {
	UsersService::create_user(&state, payload).await
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/update/{id}",
	request_body = UsersUpdateRequestDto,
	responses(
		(status = 200, description = "Update user", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn put_update_user(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	UsersService::update_user(&state, id, payload).await
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/update/me",
	request_body = UsersUpdateRequestDto,
	responses(
		(status = 200, description = "Update user me", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn put_update_user_me(
	Extension(state): Extension<AppState>,
	header: HeaderMap,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	UsersService::update_user_me(&state, header, payload).await
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/activate/{id}",
	request_body = UsersActiveInactiveRequestDto,
	responses(
		(status = 200, description = "Set user active/inactive", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn patch_user_active_status(
	Extension(state): Extension<AppState>,
	axum::extract::Path(id): axum::extract::Path<String>,
	Json(payload): Json<UsersActiveInactiveRequestDto>,
) -> impl IntoResponse {
	UsersService::set_user_active_status(&state, id, payload).await
}

#[utoipa::path(
	delete,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/delete/{id}",
	responses(
		(status = 200, description = "Soft delete user", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn delete_user(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	UsersService::delete_user(&state, id).await
}
