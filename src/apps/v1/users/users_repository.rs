use super::{UsersActiveInactiveSchema, UsersSchema, UsersSetNewPasswordSchema};
use crate::{AppState, AuthOtpSchema, ResourceEnum};
use anyhow::{bail, Result};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_by_email(&self, email: String) -> Result<UsersSchema> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE email = $email",
			ResourceEnum::Users.to_string()
		);
		let mut response: Vec<UsersSchema> = db
			.query(sql)
			.bind(("email", email.clone()))
			.await?
			.take(0)?;

		if let Some(user) = response.pop() {
			Ok(user)
		} else {
			bail!("User not found")
		}
	}

	pub async fn query_user_by_id(&self, id: String) -> Result<UsersSchema> {
		let db = &self.state.surrealdb_ws;
		let result = db
			.select((ResourceEnum::Users.to_string(), id.clone()))
			.await?;
		match result {
			Some(response) => Ok(response),
			None => bail!("User not found"),
		}
	}

	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.create(ResourceEnum::Users.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	pub async fn query_update_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.update((ResourceEnum::Users.to_string(), &data.id.id.to_string()))
			.merge(data)
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_active_inactive_user(
		&self,
		email: String,
		data: UsersActiveInactiveSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_email(email.clone()).await?;
		let table = user.id.tb.as_str();
		let id = user.id.id.to_string();
		let result: Option<AuthOtpSchema> = db
			.update((table, id))
			.merge(UsersActiveInactiveSchema {
				is_active: data.is_active,
			})
			.await?;
		match result {
			Some(_) => Ok("Success update user".to_string()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_update_password_user(
		&self,
		data: UsersSetNewPasswordSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
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
