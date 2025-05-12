use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct FlagsItemDto {
	pub id: String,
	pub name: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub description: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct FlagsRequestDto {
	pub name: String,
	pub is_active: bool,
	pub description: String,
}
