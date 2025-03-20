use super::{UsersActiveInactiveSchema, UsersSchema, UsersSetNewPasswordSchema};
use crate::{get_iso_date, AppState, ResourceEnum};
use anyhow::{bail, Result};
use surrealdb::Uuid;

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
			None => {
				bail!("User not found")
			}
		}
	}

	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let id = Uuid::new_v4().to_string();
		let db = &self.state.surrealdb;

		let record: Option<UsersSchema> = db
			.create((ResourceEnum::Users.to_string(), &id))
			.content(UsersSchema {
				id: Some(id.clone()),
				role_id: data.role_id.clone(),
				fullname: data.fullname.clone(),
				email: data.email.clone(),
				password: data.password.clone(),
				avatar: data.avatar.clone(),
				phone_number: data.phone_number.clone(),
				referral_code: data.referral_code.clone(),
				referred_by: data.referred_by.clone(),
				identity_number: data.identity_number.clone(),
				student_type: data.student_type.clone(),
				religion: data.religion.clone(),
				gender: data.gender.clone(),
				birthdate: data.birthdate.clone(),
				is_active: data.is_active.clone(),
				is_profile_completed: data.is_profile_completed.clone(),
				role: data.role.clone(),
				created_at: Some(get_iso_date()),
				updated_at: Some(get_iso_date()),
			})
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
