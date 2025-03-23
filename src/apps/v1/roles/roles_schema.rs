use serde::{Deserialize, Serialize};
use surrealdb::{
	sql::{Id, Thing},
	Uuid,
};

use crate::ResourceEnum;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub permissions: Vec<Thing>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for RolesSchema {
	fn default() -> Self {
		RolesSchema {
			id: Thing::from((
				ResourceEnum::Roles.to_string(),
				Id::String(Uuid::new_v4().to_string()),
			)),
			permissions: vec![Thing::from((
				ResourceEnum::Permissions.to_string(),
				Id::String(Uuid::new_v4().to_string()),
			))],
			name: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}
