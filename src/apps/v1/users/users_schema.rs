use crate::{get_iso_date, ResourceEnum};
use serde::{Deserialize, Serialize};
use surrealdb::{
	sql::{Id, Thing},
	Uuid,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub password: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub student_type: String,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub is_profile_completed: bool,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		UsersSchema {
			id: Thing::from((
				ResourceEnum::Users.to_string(),
				Id::String(Uuid::new_v4().to_string()),
			)),
			fullname: String::new(),
			email: String::new(),
			password: String::new(),
			avatar: None,
			phone_number: String::new(),
			referral_code: None,
			referred_by: None,
			identity_number: None,
			is_active: false,
			is_deleted: false,
			student_type: String::new(),
			religion: None,
			gender: None,
			birthdate: None,
			is_profile_completed: false,
			role: Thing::from((
				ResourceEnum::Roles.to_string(),
				Id::String(Uuid::new_v4().to_string()),
			)),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSetNewPasswordSchema {
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersActiveInactiveSchema {
	pub is_active: bool,
}
