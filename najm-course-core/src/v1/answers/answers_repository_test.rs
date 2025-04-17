use super::*;
use crate::create_mock_app_state;
use surrealdb::Uuid;

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
async fn test_query_create_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let result = repo.query_create(payload).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_by_id_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let id = repo.query_create(payload.clone()).await.unwrap();
	let result = repo.query_by_id(&id).await;
	assert!(result.is_ok());
	let data = result.unwrap();
	assert_eq!(data.is_correct, payload.is_correct);
}

#[tokio::test]
async fn test_query_by_id_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let result = repo.query_by_id("non-existent-id").await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_list_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create(payload).await.unwrap();
	let result = repo.query_list(Default::default()).await;
	assert!(result.is_ok());
	let data = result.unwrap().data;
	assert!(!data.is_empty());
}

#[tokio::test]
async fn test_query_update_should_modify_data() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let id = repo.query_create(payload.clone()).await.unwrap();

	let update = AnswersUpdateRequestDto {
		id: id.clone(),
		option: format!("updated-option-{}", Uuid::new_v4()),
		is_correct: false,
	};
	let result = repo.query_update(id.clone(), update.clone()).await;
	assert!(result.is_ok());

	let updated = repo.query_by_id(&id).await.unwrap();
	assert_eq!(updated.option, update.option);
	assert_eq!(updated.is_correct, update.is_correct);
}

#[tokio::test]
async fn test_query_update_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let update = AnswersUpdateRequestDto {
		option: "some-option".into(),
		is_correct: true,
		id: "non-existent-id".into(),
	};
	let result = repo.query_update("non-existent-id".into(), update).await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_delete_should_soft_delete() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let id = repo.query_create(payload).await.unwrap();
	let result = repo.query_delete(id.clone()).await;
	assert!(result.is_ok());

	let check = repo.query_by_id(&id).await;
	assert!(
		check.is_err(),
		"Expected query_by_id to fail after deletion"
	);
}

#[tokio::test]
async fn test_query_delete_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let payload = generate_payload();
	let id = repo.query_create(payload).await.unwrap();
	let _ = repo.query_delete(id.clone()).await.unwrap();
	let result = repo.query_delete(id.clone()).await;
	assert!(result.is_err());
}
