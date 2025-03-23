use super::{UsersActiveInactiveSchema, UsersSchema, UsersSetNewPasswordSchema};
use crate::{
	get_id, make_thing, AppState, CountResult, MetaRequestDto, MetaResponseDto,
	ResourceEnum, ResponseListSuccessDto,
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
	) -> Result<ResponseListSuccessDto<Vec<UsersSchema>>> {
		let db = &self.state.surrealdb_ws;
		let table = ResourceEnum::Users.to_string();

		let page = meta.page.unwrap_or(1);
		let per_page = meta.per_page.unwrap_or(10);

		if page < 1 || per_page < 1 {
			bail!("Invalid pagination: page and per_page must be greater than 0");
		}

		let start = (page - 1) * per_page;

		let mut sql = format!("SELECT * FROM {}", table);
		let mut conditions = vec!["is_deleted = false".to_string()];

		if meta.search.is_some() {
			conditions.push("string::contains(fullname, $search)".to_string());
		}

		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}

		if !conditions.is_empty() {
			sql.push_str(" WHERE ");
			sql.push_str(&conditions.join(" AND "));
		}

		if let Some(sort_by) = &meta.sort_by {
			let order = match meta
				.order
				.clone()
				.unwrap_or_else(|| "ASC".to_string())
				.to_uppercase()
				.as_str()
			{
				"ASC" => "ASC",
				"DESC" => "DESC",
				_ => "ASC", // fallback kalau order invalid
			};
			sql.push_str(&format!(" ORDER BY {} {}", sort_by, order));
		}

		sql.push_str(" LIMIT $per_page START $start");

		let mut query = db.query(sql);

		if let Some(search) = meta.search.clone() {
			query = query.bind(("search", search));
		}

		if let Some(filter_val) = meta.filter.clone() {
			if let Ok(b) = filter_val.parse::<bool>() {
				query = query.bind(("filter", b));
			} else if let Ok(i) = filter_val.parse::<i64>() {
				query = query.bind(("filter", i));
			} else {
				query = query.bind(("filter", filter_val));
			}
		}

		query = query.bind(("per_page", per_page)).bind(("start", start));

		let users: Vec<UsersSchema> = query.await?.take(0)?;

		let mut count_sql =
			format!("SELECT count() FROM {} WHERE is_deleted = false", table);
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			count_sql.push_str(&format!(" AND {} = $filter", filter_by));
		}

		let mut count_query = db.query(count_sql);

		if let Some(filter_val) = meta.filter.clone() {
			if let Ok(b) = filter_val.parse::<bool>() {
				count_query = count_query.bind(("filter", b));
			} else if let Ok(i) = filter_val.parse::<i64>() {
				count_query = count_query.bind(("filter", i));
			} else {
				count_query = count_query.bind(("filter", filter_val));
			}
		}

		let count_result: Vec<CountResult> = count_query.await?.take(0)?;
		let total = count_result.first().map(|c| c.count);

		let meta_res = MetaResponseDto {
			page: Some(page),
			per_page: Some(per_page),
			total,
		};

		Ok(ResponseListSuccessDto {
			data: users,
			meta: Some(meta_res),
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
