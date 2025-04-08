use super::QuestionsSchema;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct QuestionsRequestDto {
	#[validate(length(min = 1, message = "Question must not be empty"))]
	pub question: String,

	#[validate(length(min = 1, message = "Discussion must not be empty"))]
	pub discussion: String,

	pub question_image_url: Option<String>,

	#[validate(length(min = 1, message = "Discussion image URL must not be empty"))]
	pub discussion_image_url: String,

	#[validate(length(min = 1, message = "Options must contain at least 1 ID"))]
	pub options: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub question_image_url: Option<String>,
	pub discussion_image_url: String,
	pub options: Vec<String>,
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

impl From<QuestionsSchema> for QuestionsItemDto {
	fn from(value: QuestionsSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		let options = value
			.options
			.into_iter()
			.map(|thing: Thing| match thing.id {
				surrealdb::sql::Id::String(s) => s,
				_ => "".to_string(),
			})
			.collect();

		QuestionsItemDto {
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
