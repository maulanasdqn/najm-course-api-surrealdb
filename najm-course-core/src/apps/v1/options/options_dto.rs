use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::OptionsSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsRequestDto {
	pub id: String,
	pub label: String,
	pub image_url: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub id: String,
	pub label: String,
	pub image_url: Option<String>,
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
