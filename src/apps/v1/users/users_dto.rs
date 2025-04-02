use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

use crate::{RolesItemDto, RolesItemDtoRaw};

lazy_static! {
	static ref PASSWORD_REGEX: Regex = Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersActiveInactiveRequestDto {
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersCreateRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(regex(
		path = "PASSWORD_REGEX",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,
	#[validate(length(min = 1, message = "Student type is required"))]
	pub student_type: String,
	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,
	#[validate(length(
		max = 4,
		message = "Referal code cannot be more than 4 character"
	))]
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub is_active: bool,
	pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersUpdateRequestDto {
	#[validate(email(message = "Email not valid"))]
	pub email: Option<String>,
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: Option<String>,
	#[validate(length(min = 1, message = "Student type is required"))]
	pub student_type: Option<String>,
	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: Option<String>,
	#[validate(length(
		max = 4,
		message = "Referal code cannot be more than 4 character"
	))]
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub is_active: Option<bool>,
	#[validate(length(min = 16, message = "NIK at least have 16 character"))]
	pub identity_number: Option<String>,
	#[validate(length(min = 1, message = "Religion is required"))]
	pub religion: Option<String>,
	#[validate(length(min = 1, message = "Gender is required"))]
	pub gender: Option<String>,
	#[validate(length(min = 1, message = "Birthdate is required"))]
	pub birthdate: Option<String>,
	#[validate(length(min = 1, message = "Avatar is required"))]
	pub avatar: Option<String>,
	pub role_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemDto {
	pub id: String,
	pub role: RolesItemDto,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referred_by: Option<String>,
	pub referral_code: Option<String>,
	pub student_type: String,
	pub is_active: bool,
	pub is_profile_completed: bool,
	pub is_deleted: bool,
	pub identity_number: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub password: String,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailItemDto {
	pub id: String,
	pub role: RolesItemDto,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referred_by: Option<String>,
	pub referral_code: Option<String>,
	pub student_type: String,
	pub is_active: bool,
	pub is_profile_completed: bool,
	pub identity_number: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersListItemDto {
	pub id: String,
	pub role: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referred_by: Option<String>,
	pub referral_code: Option<String>,
	pub student_type: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersListItemDtoRaw {
	pub id: Thing,
	pub role: Option<String>,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referred_by: Option<String>,
	pub referral_code: Option<String>,
	pub student_type: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersItemDtoRaw {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referred_by: Option<String>,
	pub referral_code: Option<String>,
	pub student_type: String,
	pub is_active: bool,
	pub is_profile_completed: bool,
	pub is_deleted: bool,
	pub identity_number: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub role: RolesItemDtoRaw,
	pub password: String,
	pub created_at: String,
	pub updated_at: String,
}
