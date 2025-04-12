use super::QuestionsSchema;
use crate::{OptionsCreateRequestDto, OptionsItemDto, OptionsUpdateRequestDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct QuestionsCreateRequestDto {
	#[validate(length(min = 1, message = "Question must not be empty"))]
	pub question: String,

	#[validate(length(min = 1, message = "Discussion must not be empty"))]
	pub discussion: String,

	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,

	#[validate(length(min = 1, message = "At least one option is required"))]
	#[validate]
	pub options: Vec<OptionsCreateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct QuestionsUpdateRequestDto {
	#[validate(length(min = 1, message = "Question ID is required"))]
	pub id: String,

	#[validate(length(min = 1, message = "Question must not be empty"))]
	pub question: String,

	#[validate(length(min = 1, message = "Discussion must not be empty"))]
	pub discussion: String,

	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,

	#[validate(length(min = 1, message = "At least one option is required"))]
	#[validate]
	pub options: Vec<OptionsUpdateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<OptionsItemDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsResponseListDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub created_at: String,
	pub updated_at: String,
}

impl From<QuestionsSchema> for QuestionsResponseListDto {
	fn from(value: QuestionsSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		QuestionsResponseListDto {
			id,
			question: value.question,
			discussion: value.discussion,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl QuestionsItemDto {
	pub fn from_with_options(
		value: QuestionsSchema,
		options: Vec<OptionsItemDto>,
	) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		Self {
			id,
			question: value.question,
			discussion: value.discussion,
			question_image_url: value.question_image_url,
			discussion_image_url: value.discussion_image_url,
			options,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
