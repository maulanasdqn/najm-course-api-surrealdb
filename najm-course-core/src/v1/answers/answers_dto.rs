use super::answers_schema::AnswersSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AnswersCreateRequestDto {
	#[validate(length(min = 1, message = "User ID must not be empty"))]
	#[schema(example = "user:abc123")]
	pub user: String,

	#[validate(length(min = 1, message = "Test ID must not be empty"))]
	#[schema(example = "test:xyz456")]
	pub test: String,

	#[validate(length(min = 1, message = "Question ID must not be empty"))]
	#[schema(example = "question:q1")]
	pub question: String,

	#[validate(length(min = 1, message = "Option ID must not be empty"))]
	#[schema(example = "option:o1")]
	pub option: String,

	#[schema(example = true)]
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AnswersUpdateRequestDto {
	#[validate(length(min = 1, message = "Answer ID must not be empty"))]
	#[schema(example = "answer:123abc")]
	pub id: String,

	#[validate(length(min = 1, message = "Option ID must not be empty"))]
	#[schema(example = "option:o2")]
	pub option: String,

	#[schema(example = false)]
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AnswersResponseDto {
	pub id: String,
	pub user: String,
	pub test: String,
	pub question: String,
	pub option: String,
	pub is_correct: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<AnswersSchema> for AnswersResponseDto {
	fn from(value: AnswersSchema) -> Self {
		Self {
			id: value.id.id.to_raw(),
			user: value.user.id.to_raw(),
			test: value.test.id.to_raw(),
			question: value.question.id.to_raw(),
			option: value.option.id.to_raw(),
			is_correct: value.is_correct,
			is_deleted: value.is_deleted,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
