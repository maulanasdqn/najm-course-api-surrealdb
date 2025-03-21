use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::ResourceEnum;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: String,
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
	pub is_profile_completed: bool,
	pub role: Thing,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for UsersSchema {
	fn default() -> Self {
		UsersSchema {
			id: String::new(),
			role_id: String::new(),
			fullname: String::new(),
			email: String::new(),
			password: String::new(),
			avatar: None,
			phone_number: String::new(),
			referral_code: None,
			referred_by: None,
			identity_number: None,
			is_active: false,
			student_type: String::new(),
			religion: None,
			gender: None,
			birthdate: None,
			role: Thing::from((
				ResourceEnum::Roles.to_string(),
				Id::String("".to_string()),
			)),
			is_profile_completed: false,
			created_at: None,
			updated_at: None,
		}
	}
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
