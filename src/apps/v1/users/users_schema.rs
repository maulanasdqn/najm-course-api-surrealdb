use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersSchema {
	pub id: String,
	pub email: String,
	pub fullname: String,
	pub password: String,
}
