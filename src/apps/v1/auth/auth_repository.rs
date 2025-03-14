use crate::{v1::UsersItemDto, AppState, RedisKeyEnum, ResourceEnum};
use anyhow::{bail, Result};
use redis::Commands;

use super::{AuthQueryByEmailResponse, AuthRegisterRequestDto};

pub struct AuthRepository<'a> {
	state: &'a AppState,
}

impl<'a> AuthRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub fn query_store_user_data(
		&self,
		user: AuthRegisterRequestDto,
	) -> Result<String> {
		let redis_key = format!("{}:{}", RedisKeyEnum::User, user.email.clone());
		match &self.state.redisdb.get_connection().and_then(|mut conn| {
			conn.set_ex::<_, String, ()>(
				&redis_key,
				serde_json::to_string(&user).unwrap_or_default(),
				86400,
			)
		}) {
			Ok(_) => Ok("Success store user data".to_string()),
			Err(err) => Ok(format!("Redis storage failed: {}", err)),
		}
	}

	pub fn query_get_stored_user(&self, email: String) -> Result<UsersItemDto> {
		let redis_key = format!("{}:{}", RedisKeyEnum::User, email);
		let mut conn = self.state.redisdb.get_connection()?;

		let data: Option<String> = conn.get(&redis_key)?;

		match data {
			Some(user_json) => {
				let user: UsersItemDto = serde_json::from_str(&user_json)?;
				Ok(user)
			}
			None => bail!("No stored user data found for email"),
		}
	}

	pub async fn query_user_by_email(
		&self,
		email: String,
	) -> Result<AuthQueryByEmailResponse> {
		let db = &self.state.surrealdb;

		let result = db.select((ResourceEnum::Users.to_string(), email)).await?;

		match result {
			Some(response) => Ok(response),
			None => bail!("User not found"),
		}
	}

	pub async fn query_create_user(
		&self,
		data: AuthRegisterRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb;

		let record: Option<UsersItemDto> = db
			.create((ResourceEnum::Users.to_string(), &data.email))
			.content(data)
			.await?;

		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}
}
