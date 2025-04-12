use crate::{
	create_mock_app_state,
	v1::{
		options::{OptionsCreateRequestDto, OptionsUpdateRequestDto},
		questions::{
			QuestionsCreateRequestDto, QuestionsRepository, QuestionsUpdateRequestDto,
		},
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
		discussion: "This is a discussion".into(),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![
			generate_option("Option A", false),
			generate_option("Option B", true),
		],
	}
}

#[tokio::test]
async fn test_create_question_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let res = repo.query_create_question(payload).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_question_list_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let res = repo.query_question_list(Default::default()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_question_by_id_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let _ = repo.query_create_question(payload.clone()).await.unwrap();
	let all = repo.query_question_list(Default::default()).await.unwrap();
	let latest = all.data.last().expect("Expected at least one question");
	let res = repo.query_question_by_id(latest.id.clone()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_update_question_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let _ = repo.query_create_question(payload.clone()).await.unwrap();
	let all = repo.query_question_list(Default::default()).await.unwrap();
	let latest = all.data.last().expect("Expected at least one question");
	let update = QuestionsUpdateRequestDto {
		id: latest.id.clone(),
		question: format!("Updated {}", payload.question),
		discussion: "Updated discussion".into(),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![
			OptionsUpdateRequestDto {
				id: "".into(),
				label: "Updated A".into(),
				image_url: None,
				is_correct: false,
			},
			OptionsUpdateRequestDto {
				id: "".into(),
				label: "Updated B".into(),
				image_url: None,
				is_correct: true,
			},
		],
	};
	let res = repo.query_update_question(latest.id.clone(), update).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_delete_question_should_soft_delete() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let create_result = repo.query_create_question(payload.clone()).await;
	assert!(create_result.is_ok(), "Failed to create question");
	let id = create_result.unwrap();
	let delete_result = repo.query_delete_question(id.clone()).await;
	assert!(
		delete_result.is_ok(),
		"Failed to delete question: {:?}",
		delete_result.unwrap_err()
	);
}

#[tokio::test]
async fn test_delete_question_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let _ = repo.query_create_question(payload.clone()).await.unwrap();
	let all = repo.query_question_list(Default::default()).await.unwrap();
	let latest = all.data.last().expect("Expected at least one question");
	let _ = repo.query_delete_question(latest.id.clone()).await.unwrap();
	let res = repo.query_delete_question(latest.id.clone()).await;
	assert!(res.is_err());
}
