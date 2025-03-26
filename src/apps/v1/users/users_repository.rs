use super::{
	UsersActiveInactiveSchema, UsersItemDto, UsersItemDtoRaw, UsersSchema,
	UsersSetNewPasswordSchema,
};
use crate::{
	get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto, ResourceEnum,
	ResponseListSuccessDto,
};
use anyhow::{bail, Result};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<UsersItemDto>>> {
		let db = &self.state.surrealdb_ws;

		let mut conditions = vec!["is_deleted = false".to_string()];

		if let Some(search) = meta.search.as_deref() {
			if !search.is_empty() {
				conditions.push("string::contains(fullname ?? '', $search)".to_string());
			}
		}

		if let (Some(filter_by), Some(_filter)) =
			(meta.filter_by.as_ref(), meta.filter.as_ref())
		{
			conditions.push(format!("{} = $filter", filter_by));
		}

		let where_clause = if !conditions.is_empty() {
			format!("WHERE {}", conditions.join(" AND "))
		} else {
			String::new()
		};

		let limit = meta.per_page.unwrap_or(10);
		let start = (meta.page.unwrap_or(1) - 1) * limit;

		let select_query = format!(
			"
			SELECT
				id,
				role,
				fullname,
				email,
				avatar,
				phone_number,
				referred_by,
				referral_code,
				student_type,
				is_active,
				is_profile_completed,
				identity_number,
				religion,
				gender,
				birthdate
			FROM {}
			{}
			LIMIT {} START {}
			FETCH role, role.permissions
			",
			ResourceEnum::Users.to_string(),
			where_clause,
			limit,
			start
		);

		let raw_result = query_list_with_meta::<UsersItemDtoRaw>(
			db,
			&ResourceEnum::Users.to_string(),
			&meta,
			vec![],
			Some(select_query),
		)
		.await?;

		let converted = raw_result
			.data
			.into_iter()
			.map(UsersItemDto::from)
			.collect();

		Ok(ResponseListSuccessDto {
			data: converted,
			meta: raw_result.meta,
		})
	}

	pub async fn query_user_by_email(&self, email: String) -> Result<UsersSchema> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE email = $email AND is_deleted = false",
			ResourceEnum::Users.to_string()
		);
		let mut response: Vec<UsersSchema> = db
			.query(sql)
			.bind(("email", email.clone()))
			.await?
			.take(0)?;
		match response.pop() {
			Some(user) => Ok(user),
			None => bail!("User not found"),
		}
	}

	pub async fn query_user_by_id(&self, id: String) -> Result<UsersSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<UsersSchema> = db
			.select((ResourceEnum::Users.to_string(), id.clone()))
			.await?;
		match result {
			Some(response) if !response.is_deleted => Ok(response),
			_ => bail!("User not found"),
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
		let record_key = get_id(&data.id)?;
		let existing = self.query_user_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("User already deleted");
		}
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
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
		if user.is_deleted {
			bail!("User not found");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(UsersActiveInactiveSchema {
				is_active: data.is_active,
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_active_inactive_user_by_id(
		&self,
		id: String,
		data: UsersActiveInactiveSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.update((ResourceEnum::Users.to_string(), id))
			.merge(UsersActiveInactiveSchema {
				is_active: data.is_active,
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_update_password_user(
		&self,
		email: String,
		data: UsersSetNewPasswordSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_email(email).await?;
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSetNewPasswordSchema> = db
			.update(record_key)
			.merge(UsersSetNewPasswordSchema {
				password: data.password.clone(),
			})
			.await?;
		match record {
			Some(_) => Ok("Success update password user".into()),
			None => bail!("Failed to update password user"),
		}
	}

	pub async fn query_delete_user(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		let user = self.query_user_by_id(user_id.id.to_raw()).await?;
		if user.is_deleted {
			bail!("User already deleted");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete user".into()),
			None => bail!("Failed to delete user"),
		}
	}
}
