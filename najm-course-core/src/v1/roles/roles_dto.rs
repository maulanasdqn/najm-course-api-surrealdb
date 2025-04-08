use crate::{PermissionsItemDto, PermissionsItemDtoRaw};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

use super::RolesSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RolesRequestUpdateDto {
	#[validate(length(min = 1, message = "Role name must not be empty"))]
	pub name: Option<String>,
	pub permissions: Option<Vec<String>>,
	pub overwrite: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RolesRequestCreateDto {
	#[validate(length(min = 1, message = "Role name must not be empty"))]
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
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesItemByIdDto {
	pub id: String,
	pub name: String,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesItemByIdDtoRaw {
	pub id: Thing,
	pub name: String,
	pub permissions: Vec<PermissionsItemDtoRaw>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesItemDtoRaw {
	pub id: Thing,
	pub name: String,
	pub permissions: Vec<PermissionsItemDtoRaw>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RolesResponseDto {
	pub id: String,
	pub name: String,
	pub permissions_count: usize,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl From<RolesSchema> for RolesResponseDto {
	fn from(value: RolesSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};

		let permissions_count = value.permissions.len();

		RolesResponseDto {
			id,
			name: value.name,
			permissions_count,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
