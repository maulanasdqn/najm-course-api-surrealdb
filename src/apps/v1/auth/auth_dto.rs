use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginRequestDto {
	pub email: String,
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginResponsetDto {
	pub token: TokenDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenDto {
	pub access_token: String,
	pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthRegisterRequestDto {
	pub email: String,
	pub password: String,
	pub fullname: String,
}
