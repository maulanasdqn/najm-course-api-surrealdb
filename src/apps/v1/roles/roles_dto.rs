use crate::PermissionsItemDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesRequestUpdateDto {
	pub name: Option<String>,
	pub permissions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesRequestCreateDto {
	pub name: String,
	pub permissions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesItemListDto {
	pub id: String,
	pub name: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
