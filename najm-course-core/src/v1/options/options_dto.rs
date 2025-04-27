use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::OptionsSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct OptionsCreateRequestDto {
	#[validate(length(min = 1, message = "Label must not be empty"))]
	pub label: String,
	pub image_url: Option<String>,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct OptionsUpdateRequestDto {
	pub id: String,
	#[validate(length(min = 1, message = "Label must not be empty"))]
	pub label: String,
	pub image_url: Option<String>,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub id: String,
	pub label: String,
	pub image_url: Option<String>,
	pub is_correct: Option<bool>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsResponseListDto {
	pub id: String,
	pub label: String,
	pub image_url: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

impl From<OptionsSchema> for OptionsResponseListDto {
	fn from(value: OptionsSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		OptionsResponseListDto {
			id,
			label: value.label,
			image_url: value.image_url,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl From<OptionsSchema> for OptionsItemDto {
	fn from(o: OptionsSchema) -> Self {
		Self {
			id: match o.id.id {
				surrealdb::sql::Id::String(s) => s,
				_ => "".to_string(),
			},
			label: o.label,
			is_correct: Some(o.is_correct),
			image_url: o.image_url,
			created_at: o.created_at,
			updated_at: o.updated_at,
		}
	}
}
