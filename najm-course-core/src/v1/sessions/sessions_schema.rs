use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestSessionsSchema {
	pub test: Thing,
	pub weight: u32,
	pub multiplier: f32,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionsSchema {
	pub id: Thing,
	pub name: String,
	pub tests: Vec<TestSessionsSchema>,
	pub category: String,
	pub description: String,
	pub student_type: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
