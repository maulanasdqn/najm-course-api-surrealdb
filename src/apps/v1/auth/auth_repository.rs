use crate::{AppState, ResourceEnum, UsersSchema};
use anyhow::{anyhow, bail, Result};
use chrono::{Duration, Utc};

use super::AuthOtpSchema;

pub struct AuthRepository<'a> {
	state: &'a AppState,
}

impl<'a> AuthRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_store_user(&self, user: UsersSchema) -> Result<String> {
		let user_clone = user.clone();
		let record: Option<UsersSchema> = self
			.state
			.surrealdb_mem
			.update((ResourceEnum::UsersCache.to_string(), user.email))
			.content(user_clone)
			.await?;
		match record {
			Some(_) => Ok("Success store user data".to_string()),
			None => bail!("Failed store user data"),
		}
	}

	pub async fn query_get_stored_user(&self, email: String) -> Result<UsersSchema> {
		let user: Option<UsersSchema> = self
			.state
			.surrealdb_mem
			.select((ResourceEnum::UsersCache.to_string(), email))
			.await?;
		match user {
			Some(u) => Ok(u),
			None => bail!("No stored user data found"),
		}
	}

	pub async fn query_delete_stored_user(&self, email: String) -> Result<String> {
		let record: Option<String> = self
			.state
			.surrealdb_mem
			.delete((ResourceEnum::UsersCache.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete stored user".to_string()),
			None => bail!("Failed delete stored user"),
		}
	}

	pub async fn query_get_stored_otp(&self, email: String) -> Result<u32> {
		let otp: Option<AuthOtpSchema> = self
			.state
			.surrealdb_mem
			.select((ResourceEnum::OtpCache.to_string(), &email))
			.await?;
		match otp {
			Some(data) => {
				if Utc::now() > data.expires_at {
					let _: Option<AuthOtpSchema> = self
						.state
						.surrealdb_mem
						.delete((ResourceEnum::OtpCache.to_string(), &email))
						.await?;
					Err(anyhow!("OTP expired"))
				} else {
					Ok(data.otp)
				}
			}
			None => Err(anyhow!("No stored OTP found")),
		}
	}

	pub async fn query_store_otp(&self, email: String, otp: u32) -> Result<String> {
		let expires_at = Utc::now() + Duration::seconds(300); // 5 menit
		let record: Option<AuthOtpSchema> = self
			.state
			.surrealdb_mem
			.create((ResourceEnum::OtpCache.to_string(), email))
			.content(AuthOtpSchema { otp, expires_at })
			.await?;
		match record {
			Some(_) => Ok("Success store otp".to_string()),
			None => bail!("Failed store otp"),
		}
	}

	pub async fn query_delete_stored_otp(&self, email: String) -> Result<String> {
		let record: Option<String> = self
			.state
			.surrealdb_mem
			.delete((ResourceEnum::OtpCache.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete stored otp".to_string()),
			None => bail!("Failed delete stored otp"),
		}
	}
}
