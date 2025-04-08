use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestsSchema {
	pub id: Thing,
	pub name: String,
	pub questions: Vec<Thing>,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
