use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaSchema {
	pub transaction_number: String,
	pub user: Thing,
}
