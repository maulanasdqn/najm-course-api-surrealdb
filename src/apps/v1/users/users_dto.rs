use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersCreateRequestDto {
	pub email: String,
	pub fullname: String,
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemDto {
	pub id: String,
	pub fullname: String,
	pub email: String,
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
}
