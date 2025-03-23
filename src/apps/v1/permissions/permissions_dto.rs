use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsRequestDto {
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsItemDto {
	pub id: String,
	pub name: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
