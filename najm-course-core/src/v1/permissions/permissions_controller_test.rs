use crate::{
	create_mock_app_state, permissions_router,
	v1::{
		permissions_dto::PermissionsRequestDto,
		permissions_repository::PermissionsRepository,
	},
	AppState, PermissionsEnum,
};
use axum::{http::StatusCode, Extension, Router};
use axum_test::TestServer;
use najm_course_utils::authorized;

pub fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/permissions", permissions_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

async fn create_dummy_permission(
	repo: &PermissionsRepository<'_>,
	name: &str,
) -> String {
	let dto = crate::v1::permissions_schema::PermissionsSchema {
		name: name.into(),
		..Default::default()
	};
	let _ = repo.query_create_permission(dto.clone()).await.unwrap();
	let found = repo.query_permission_by_name(name.into()).await.unwrap();
	found.id.id.to_raw()
}

async fn delete_dummy_permission(repo: &PermissionsRepository<'_>, id: String) {
	let _ = repo.query_delete_permission(id).await;
}

#[tokio::test]
async fn test_create_permission_should_return_201() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let server = create_test_app(state.clone());
	let payload = PermissionsRequestDto {
		name: "Create Permission".into(),
	};
	let res = authorized(
		&server,
		"POST",
		"/v1/permissions/create",
		vec![&PermissionsEnum::CreatePermissions.to_string()],
		Some(&payload),
	)
	.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::CREATED);
	let id = repo
		.query_permission_by_name(payload.name.clone())
		.await
		.unwrap()
		.id
		.id
		.to_raw();
	delete_dummy_permission(&repo, id).await;
}

#[tokio::test]
async fn test_get_permission_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/permissions?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListPermissions.to_string()],
		None,
	)
	.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_permission_by_id_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let id = create_dummy_permission(&repo, "Get Permission").await;
	let server = create_test_app(state.clone());

	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/permissions/detail/{}", id),
		vec![&PermissionsEnum::ReadDetailPermissions.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
	delete_dummy_permission(&repo, id).await;
}

#[tokio::test]
async fn test_update_permission_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let id = create_dummy_permission(&repo, "To Update").await;
	let server = create_test_app(state.clone());
	let payload = PermissionsRequestDto {
		name: "Updated Permission".into(),
	};

	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/permissions/update/{}", id),
		vec![&PermissionsEnum::UpdatePermissions.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
	delete_dummy_permission(&repo, id).await;
}

#[tokio::test]
async fn test_delete_permission_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let id = create_dummy_permission(&repo, "To Delete").await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/permissions/delete/{}", id),
		vec![&PermissionsEnum::DeletePermissions.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_duplicate_permission_should_return_409() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let name = "Duplicate Permission";
	let id = create_dummy_permission(&repo, name).await;
	let server = create_test_app(state.clone());
	let payload = PermissionsRequestDto { name: name.into() };

	let res = authorized(
		&server,
		"POST",
		"/v1/permissions/create",
		vec![&PermissionsEnum::CreatePermissions.to_string()],
		Some(&payload),
	)
	.await;

	assert_eq!(res.status_code(), StatusCode::CONFLICT);
	delete_dummy_permission(&repo, id).await;
}

#[tokio::test]
async fn test_delete_nonexistent_permission_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"DELETE",
		"/v1/permissions/delete/nonexistent-id",
		vec![&PermissionsEnum::DeletePermissions.to_string()],
		None,
	)
	.await;

	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_permission_with_empty_name_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = PermissionsRequestDto { name: "".into() };

	let res = authorized(
		&server,
		"POST",
		"/v1/permissions/create",
		vec![&PermissionsEnum::CreatePermissions.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_nonexistent_permission_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = PermissionsRequestDto {
		name: "Nonexistent".into(),
	};

	let res = authorized(
		&server,
		"PUT",
		"/v1/permissions/update/non-exist",
		vec![&PermissionsEnum::UpdatePermissions.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_permission_by_invalid_id_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/permissions/detail/invalid-id",
		vec![&PermissionsEnum::ReadDetailPermissions.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_permission_list_with_invalid_pagination_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/permissions?page=abc&per_page=xyz",
		vec![&PermissionsEnum::ReadListPermissions.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}
