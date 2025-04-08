use super::tests_schema::TestsSchema;
use crate::{
	questions::QuestionsItemDto, QuestionsCreateRequestDto, QuestionsUpdateRequestDto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestsCreateRequestDto {
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,
	pub questions: Vec<QuestionsCreateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestsUpdateRequestDto {
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,
	pub questions: Vec<QuestionsUpdateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemDto {
	pub id: String,
	pub name: String,
	pub questions: Vec<QuestionsItemDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsResponseListDto {
	pub id: String,
	pub name: String,
	pub question_count: u32,
	pub created_at: String,
	pub updated_at: String,
}

impl From<TestsSchema> for TestsResponseListDto {
	fn from(value: TestsSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		TestsResponseListDto {
			id,
			name: value.name,
			question_count: value.questions.len() as u32,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl TestsItemDto {
	pub fn from_with_questions(
		value: TestsSchema,
		questions: Vec<QuestionsItemDto>,
	) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		TestsItemDto {
			id,
			name: value.name,
			questions,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
