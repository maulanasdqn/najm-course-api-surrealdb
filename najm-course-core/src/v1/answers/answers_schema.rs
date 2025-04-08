use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswersSchema {
	pub id: Thing,
	pub user: Thing,
	pub test: Thing,
	pub question: Thing,
	pub option: Thing,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
