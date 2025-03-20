use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub email: String,
	pub fullname: String,
	pub password: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSetNewPasswordSchema {
	pub email: String,
	pub password: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersActiveInactiveSchema {
	pub email: String,
	pub is_active: bool,
}
