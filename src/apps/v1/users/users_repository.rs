use super::{UsersActiveInactiveSchema, UsersSchema, UsersSetNewPasswordSchema};
use crate::{AppState, ResourceEnum};
use anyhow::{bail, Result};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_by_email(&self, email: String) -> Result<UsersSchema> {
		let db = &self.state.surrealdb;
		let result = db
			.select((ResourceEnum::Users.to_string(), email.clone()))
			.await?;
		match result {
			Some(response) => Ok(response),
			None => bail!("User not found"),
		}
	}

	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersSchema> = db
			.create((ResourceEnum::Users.to_string(), &data.id))
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	pub async fn query_active_inactive_user(
		&self,
		data: UsersActiveInactiveSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersActiveInactiveSchema> = db
			.update((ResourceEnum::Users.to_string(), &data.email))
			.merge(UsersActiveInactiveSchema {
				email: data.email.clone(),
				is_active: data.is_active.clone(),
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_update_password_user(
		&self,
		data: UsersSetNewPasswordSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersSetNewPasswordSchema> = db
			.update((ResourceEnum::Users.to_string(), &data.email))
			.merge(UsersSetNewPasswordSchema {
				email: data.email.clone(),
				password: data.password.clone(),
			})
			.await?;
		match record {
			Some(_) => Ok("Success update password user".into()),
			None => bail!("Failed to update password user"),
		}
	}
}
