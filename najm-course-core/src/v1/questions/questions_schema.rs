use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionsSchema {
	pub id: Thing,
	pub question: String,
	pub discussion: String,
	pub question_image_url: Option<String>,
	pub discussion_image_url: String,
	pub options: Vec<Thing>,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
