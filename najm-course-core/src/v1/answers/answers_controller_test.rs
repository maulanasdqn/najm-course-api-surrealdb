use super::answers_router;
use crate::{
	create_mock_app_state,
	v1::answers::{AnswersCreateRequestDto, AnswersUpdateRequestDto},
	AppState, PermissionsEnum,
};
use axum::{Extension, Router};
use axum_test::TestServer;
use najm_course_utils::authorized;
use surrealdb::Uuid;

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/answers", answers_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

fn generate_payload() -> AnswersCreateRequestDto {
	AnswersCreateRequestDto {
		user: "user1".into(),
		test: "test1".into(),
		question: "question1".into(),
		option: format!("option-{}", Uuid::new_v4()),
		is_correct: true,
	}
}

#[tokio::test]
async fn test_post_create_answer_should_return_201() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = generate_payload();
	let res = authorized(
		&server,
		"POST",
		"/v1/answers/create",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_get_answer_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/answers?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListAnswers.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_answer_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/answers/create",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::answers::AnswersRepository::new(&state);
	let list = repo.query_list(Default::default()).await.unwrap().data;
	let item = list.last().unwrap();
	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/answers/detail/{}", item.id),
		vec![&PermissionsEnum::ReadDetailAnswers.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_put_update_answer_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/answers/create",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::answers::AnswersRepository::new(&state);
	let list = repo.query_list(Default::default()).await.unwrap().data;
	let item = list.last().unwrap();
	let update = AnswersUpdateRequestDto {
		id: item.id.clone(),
		option: format!("updated-{}", Uuid::new_v4()),
		is_correct: false,
	};
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/answers/update/{}", item.id),
		vec![&PermissionsEnum::UpdateAnswers.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_delete_answer_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/answers/create",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::answers::AnswersRepository::new(&state);
	let list = repo.query_list(Default::default()).await.unwrap().data;
	let item = list.last().unwrap();
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/answers/delete/{}", item.id),
		vec![&PermissionsEnum::DeleteAnswers.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}
