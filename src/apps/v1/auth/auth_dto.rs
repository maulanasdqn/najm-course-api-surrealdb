use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::v1::UsersItemDto;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginRequestDto {
	pub email: String,
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginResponsetDto {
	pub token: TokenDto,
	pub user: UsersItemDto,
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthQueryByEmailResponse {
	pub email: String,
	pub fullname: String,
	pub password: String,
}
