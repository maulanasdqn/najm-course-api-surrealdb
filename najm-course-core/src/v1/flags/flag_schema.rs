use super::{FlagsItemDto, FlagsRequestDto};
use najm_course_libs::ResourceEnum;
use najm_course_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagsSchema {
	pub id: Thing,
	pub name: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub description: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl FlagsSchema {
	pub fn new(request: FlagsRequestDto) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Flags.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: request.name,
			description: request.description,
			is_active: request.is_active,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}

	pub fn update(id: &str, request: FlagsRequestDto) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Flags.to_string(), id),
			name: request.name,
			description: request.description,
			is_active: request.is_active,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}

	pub fn from(&self) -> FlagsItemDto {
		FlagsItemDto {
			id: self.id.id.to_raw(),
			name: self.name.clone(),
			is_active: self.is_active,
			is_deleted: self.is_deleted,
			description: self.description.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}
}
