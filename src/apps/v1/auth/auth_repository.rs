use crate::{AppState, RedisKeyEnum, UsersSchema};
use anyhow::{anyhow, bail, Result};
use redis::Commands;

pub struct AuthRepository<'a> {
	state: &'a AppState,
}

impl<'a> AuthRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub fn query_store_user_data(&self, user: UsersSchema) -> Result<String> {
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

	pub fn query_get_stored_user(&self, email: String) -> Result<UsersSchema> {
		let redis_key = format!("{}:{}", RedisKeyEnum::User, email);
		let mut conn = self.state.redisdb.get_connection()?;
		let data: Option<String> = conn.get(&redis_key)?;
		match data {
			Some(user_json) => {
				let user: UsersSchema = serde_json::from_str(&user_json)?;
				Ok(user)
			}
			None => bail!("No stored user data found"),
		}
	}

	pub fn query_get_stored_otp(&self, email: String) -> Result<u32> {
		let redis_key = format!("{}:{}", RedisKeyEnum::Otp, email);
		let mut conn = match self.state.redisdb.get_connection() {
			Ok(conn) => conn,
			Err(e) => {
				return Err(anyhow::anyhow!("Failed to get Redis connection: {}", e))
			}
		};
		let data: Option<String> = match conn.get(&redis_key) {
			Ok(data) => data,
			Err(e) => return Err(anyhow::anyhow!("Failed to get data from Redis: {}", e)),
		};
		match data {
			Some(otp_str) => match otp_str.parse::<u32>() {
				Ok(otp) => Ok(otp),
				Err(e) => Err(anyhow::anyhow!("Failed to parse OTP as u64: {}", e)),
			},
			None => Err(anyhow::anyhow!("No stored OTP found")),
		}
	}

	pub fn query_store_otp(&self, email: String, otp: u32) -> Result<String> {
		let redis_key: String = format!("{}:{}", RedisKeyEnum::Otp, email);
		let mut conn = match self.state.redisdb.get_connection() {
			Ok(conn) => conn,
			Err(e) => return Err(anyhow!("Failed to get Redis connection: {}", e)),
		};
		let otp_str: String = otp.to_string();
		match conn.set_ex::<_, _, ()>(&redis_key, &otp_str, 300) {
			Ok(_) => Ok("Success store otp".to_string()),
			Err(e) => Err(anyhow!("Failed to store OTP in Redis: {}", e)),
		}
	}

	pub fn query_delete_stored_otp(&self, email: String) -> Result<String> {
		let redis_key = format!("{}:{}", RedisKeyEnum::Otp, email);
		let mut conn = match self.state.redisdb.get_connection() {
			Ok(conn) => conn,
			Err(e) => return Err(anyhow!("Failed to get Redis connection: {}", e)),
		};
		match conn.del::<_, ()>(&redis_key) {
			Ok(_) => Ok("Successfully deleted OTP".to_string()),
			Err(e) => Err(anyhow!("Failed to delete OTP from Redis: {}", e)),
		}
	}
}
