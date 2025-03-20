use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Option<String>,
	pub role_id: String,
	pub fullname: String,
	pub email: String,
	pub password: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub student_type: String,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub is_profile_completed: Option<bool>,
	pub role: Thing,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSetNewPasswordSchema {
	pub email: String,
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersActiveInactiveSchema {
	pub email: String,
	pub is_active: bool,
}
