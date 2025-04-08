use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionsSchema {
	pub id: Thing,
	pub label: String,
	pub image_url: Option<String>,
	pub is_correct: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
