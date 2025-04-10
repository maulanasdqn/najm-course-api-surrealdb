use crate::{
	create_mock_app_state, roles_router, v1::roles_repository::RolesRepository,
	RolesRequestCreateDto, RolesRequestUpdateDto,
};
use crate::{AppState, PermissionsEnum};
use axum::http::StatusCode;
use axum::Extension;
use axum::Router;
use axum_test::TestServer;
use najm_course_utils::authorized;

pub fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/roles", roles_router())
		.layer(Extension(state));

	TestServer::new(app).unwrap()
}

async fn delete_dummy_role(state: AppState, id: String) {
	let repo = RolesRepository::new(&state);
	let _ = repo.query_delete_role(id).await;
}

#[tokio::test]
async fn test_get_role_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());

	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/roles?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListRoles.to_string()],
		None,
	)
	.await;

	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_create_role_should_return_201() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());

	let payload = RolesRequestCreateDto {
		name: "Test Role #1".into(),
		permissions: vec![],
	};

	let res = authorized(
		&server,
		"POST",
		"/v1/roles/create",
		vec![&PermissionsEnum::CreateRoles.to_string()],
		Some(&payload),
	)
	.await;

	assert_eq!(res.status_code(), StatusCode::CREATED);

	let repo = RolesRepository::new(&state);
	let id = repo
		.query_role_by_name(payload.name.clone())
		.await
		.unwrap()
		.id;
	delete_dummy_role(state, id).await;
}

#[tokio::test]
async fn test_put_update_role_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);

	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "Role To Update".into(),
			permissions: vec![],
		})
		.await
		.unwrap();

	let existing = repo
		.query_role_by_name("Role To Update".into())
		.await
		.unwrap();

	let id = existing.id.clone();
	let server = create_test_app(state.clone());

	let payload = RolesRequestUpdateDto {
		name: Some("Updated Role".into()),
		permissions: Some(vec![]),
		overwrite: None,
	};

	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/roles/update/{}", id),
		vec![&PermissionsEnum::UpdateRoles.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);

	let id = repo
		.query_role_by_name(payload.name.clone().unwrap())
		.await
		.unwrap()
		.id;
	delete_dummy_role(state.clone(), id).await;
}

#[tokio::test]
async fn test_delete_role_should_return_200() {
	let state = create_mock_app_state().await;
	let repo = RolesRepository::new(&state);

	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "Role To Delete".into(),
			permissions: vec![],
		})
		.await
		.unwrap();

	let existing = repo
		.query_role_by_name("Role To Delete".into())
		.await
		.unwrap();

	let id = existing.id;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/roles/delete/{}", id),
		vec![&PermissionsEnum::DeleteRoles.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_create_role_with_empty_name_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let payload = RolesRequestCreateDto {
		name: "".into(),
		permissions: vec![],
	};

	let res = authorized(
		&server,
		"POST",
		"/v1/roles/create",
		vec![&PermissionsEnum::CreateRoles.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_put_update_nonexistent_role_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let payload = RolesRequestUpdateDto {
		name: Some("Does Not Exist".into()),
		permissions: Some(vec![]),
		overwrite: None,
	};

	let res = authorized(
		&server,
		"PUT",
		"/v1/roles/update/nonexistent-id",
		vec![&PermissionsEnum::UpdateRoles.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_role_should_return_404() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"DELETE",
		"/v1/roles/delete/nonexistent-id",
		vec![&PermissionsEnum::DeleteRoles.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_role_list_with_invalid_page_params_should_return_400() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/roles?page=abc&per_page=xyz",
		vec![&PermissionsEnum::ReadListRoles.to_string()],
		None,
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_duplicate_role_should_return_409() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);

	let payload = RolesRequestCreateDto {
		name: "Admin".into(),
		permissions: vec![],
	};

	let res = authorized(
		&server,
		"POST",
		"/v1/roles/create",
		vec![&PermissionsEnum::CreateRoles.to_string()],
		Some(&payload),
	)
	.await;

	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::CONFLICT);
}
