use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::v1::UsersItemDto;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRequestDto {
	pub email: String,
	pub fullname: String,
	pub transaction_number: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreateItemRequestDto {
	pub item_name: String,
	pub item_image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemResponseDto {
	pub item_name: String,
	pub item_image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaResponseDto {
	pub transaction_number: String,
	pub user: UsersItemDto,
}
