use crate::{
	create_mock_app_state,
	v1::{
		options::OptionsCreateRequestDto,
		questions::QuestionsCreateRequestDto,
		tests::{TestsCreateRequestDto, TestsRepository},
	},
};
use surrealdb::Uuid;

fn generate_option(label: &str, correct: bool) -> OptionsCreateRequestDto {
	OptionsCreateRequestDto {
		label: label.into(),
		image_url: None,
		is_correct: correct,
	}
}

fn generate_question_payload() -> QuestionsCreateRequestDto {
	QuestionsCreateRequestDto {
		question: format!("Question {}", Uuid::new_v4()),
		discussion: "Discussion".into(),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![generate_option("A", false), generate_option("B", true)],
	}
}

fn generate_test_payload(name: &str) -> TestsCreateRequestDto {
	TestsCreateRequestDto {
		name: name.to_string(),
		questions: vec![generate_question_payload()],
	}
}

#[tokio::test]
async fn test_query_create_test_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Sample Test");
	let res = repo.query_create_test(payload).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_query_create_test_should_fail_if_no_questions() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let payload = TestsCreateRequestDto {
		name: "Empty".into(),
		questions: vec![],
	};
	let res = repo.query_create_test(payload).await;
	assert!(res.is_err());
}

#[tokio::test]
async fn test_query_create_test_should_fail_if_option_label_empty() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let mut question = generate_question_payload();
	question.options[0].label = "".into();
	let payload = TestsCreateRequestDto {
		name: "Invalid Option".into(),
		questions: vec![question],
	};
	let res = repo.query_create_test(payload).await;
	assert!(res.is_err());
}

#[tokio::test]
async fn test_query_test_list_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let res = repo.query_test_list(Default::default()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_query_test_by_id_should_return_test_with_questions() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Full Fetch Test");
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	let tests = repo.query_test_list(Default::default()).await.unwrap();
	let found = tests
		.data
		.iter()
		.find(|t| t.name == payload.name)
		.expect("Test not found");
	let res = repo.query_test_by_id(&found.id.clone()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_query_create_test_should_fail_if_question_text_empty() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let mut question = generate_question_payload();
	question.question = "".into();
	let payload = TestsCreateRequestDto {
		name: "Empty Question Text".into(),
		questions: vec![question],
	};
	let res = repo.query_create_test(payload).await;
	assert!(res.is_err());
}

#[tokio::test]
async fn test_query_create_test_should_fail_if_question_has_no_options() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let mut question = generate_question_payload();
	question.options.clear();
	let payload = TestsCreateRequestDto {
		name: "No Options".into(),
		questions: vec![question],
	};
	let res = repo.query_create_test(payload).await;
	assert!(res.is_err());
}

#[tokio::test]
async fn test_query_test_by_id_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let result = repo.query_test_by_id("non-existent-id".into()).await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_delete_test_should_succeed() {
	use std::time::Duration;
	use tokio::time::sleep;
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Delete Test");
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	sleep(Duration::from_millis(300)).await;
	let test = repo
		.query_test_by_name(&payload.name)
		.await
		.expect("Test not found before deletion");
	let res = repo.query_delete_test(test.id.clone()).await;
	assert!(res.is_ok(), "Failed to delete test");
}

#[tokio::test]
async fn test_query_delete_test_should_fail_if_already_deleted() {
	use std::time::Duration;
	use tokio::time::sleep;
	let state = create_mock_app_state().await;
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Delete Twice Test");
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	sleep(Duration::from_millis(300)).await;
	let test = repo
		.query_test_by_name(&payload.name.clone())
		.await
		.expect("Test not found before deletion");
	let _ = repo.query_delete_test(test.id.clone()).await.unwrap();
	let result = repo.query_delete_test(test.id.clone()).await;
	assert!(
		result.is_err(),
		"Expected error when deleting already deleted test"
	);
}
