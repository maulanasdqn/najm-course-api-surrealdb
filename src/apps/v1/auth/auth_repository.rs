use crate::{v1::UsersItemDto, AppState};
use std::error::Error;

use super::AuthRegisterRequestDto;

pub async fn query_user_by_email(
	email: String,
	state: &AppState,
) -> Result<UsersItemDto, Box<dyn Error>> {
	let db = &state.surrealdb;

	let mut result = db
		.query("SELECT * FROM app_users WHERE email = $email LIMIT 1;")
		.bind(("email", email.clone()))
		.await?;

	let user: Option<UsersItemDto> = result.take(0)?;

	user.ok_or_else(|| format!("User not found for email: {}", email).into())
}

pub async fn query_create_user(
	data: AuthRegisterRequestDto,
	state: &AppState,
) -> Result<String, Box<dyn Error>> {
	let db = &state.surrealdb;

	let _record: Option<UsersItemDto> =
		db.create(("app_users", &data.email)).content(data).await?;

	Ok("Success create user".into())
}
