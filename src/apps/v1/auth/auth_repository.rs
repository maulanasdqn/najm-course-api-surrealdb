use crate::{v1::UsersItemDto, AppState, ResourceEnum};
use std::error::Error;

use super::AuthRegisterRequestDto;

pub struct AuthRepository<'a> {
	state: &'a AppState,
}

impl<'a> AuthRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_by_email(
		&self,
		email: String,
	) -> Result<AuthRegisterRequestDto, Box<dyn Error>> {
		let db = &self.state.surrealdb;

		let result = db.select((ResourceEnum::Users.to_string(), email)).await?;

		match result {
			Some(user) => Ok(user),
			None => Err("User not found for email".into()),
		}
	}

	pub async fn query_create_user(
		&self,
		data: AuthRegisterRequestDto,
	) -> Result<String, Box<dyn Error>> {
		let db = &self.state.surrealdb;

		let record: Option<UsersItemDto> = db
			.create((ResourceEnum::Users.to_string(), &data.email))
			.content(data)
			.await?;

		match record {
			Some(_) => Ok("Success create user".into()),
			None => Err("Failed to create user".into()),
		}
	}
}
