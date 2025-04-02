use super::{PermissionsItemDto, PermissionsItemDtoRaw, PermissionsSchema};
use crate::{
	get_id, get_iso_date, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	ResourceEnum, ResponseListSuccessDto,
};
use anyhow::{bail, Result};

pub struct PermissionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> PermissionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_permission_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<PermissionsItemDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions
				.push("string::contains(string::lowercase(name ?? ''), $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw_result: ResponseListSuccessDto<Vec<PermissionsItemDtoRaw>> =
			query_list_with_meta(
				&self.state.surrealdb_ws,
				&ResourceEnum::Permissions.to_string(),
				&meta,
				conditions,
				None,
			)
			.await?;
		let clean_result = ResponseListSuccessDto {
			data: raw_result.data.into_iter().map(Into::into).collect(),
			meta: raw_result.meta,
		};

		Ok(clean_result)
	}

	pub async fn query_permission_by_id(
		&self,
		id: String,
	) -> Result<PermissionsSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<PermissionsSchema> = db
			.select((ResourceEnum::Permissions.to_string(), id.clone()))
			.await?;
		match result {
			Some(permission) if !permission.is_deleted => Ok(permission),
			_ => bail!("Permission not found"),
		}
	}

	pub async fn query_permission_by_name(
		&self,
		name: String,
	) -> Result<PermissionsSchema> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE name = $name AND is_deleted = false LIMIT 1",
			ResourceEnum::Permissions.to_string()
		);
		let result: Vec<PermissionsSchema> =
			db.query(sql).bind(("name", name.clone())).await?.take(0)?;
		if let Some(permission) = result.into_iter().next() {
			Ok(permission.into())
		} else {
			bail!("Permission not found")
		}
	}

	pub async fn query_create_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<PermissionsSchema> = db
			.create(ResourceEnum::Permissions.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create permission".into()),
			None => bail!("Failed to create permission"),
		}
	}

	pub async fn query_update_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_permission_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Permission already deleted");
		}
		let merged = PermissionsSchema {
			created_at: existing.created_at,
			updated_at: Some(get_iso_date()),
			..data.clone()
		};
		let record: Option<PermissionsSchema> =
			db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update permission".into()),
			None => bail!("Failed to update permission"),
		}
	}

	pub async fn query_delete_permission(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let permission_id = make_thing(&ResourceEnum::Permissions.to_string(), &id);
		let permission = self
			.query_permission_by_id(permission_id.id.to_raw())
			.await?;
		if permission.is_deleted {
			bail!("Permission already deleted");
		}
		let record_key = get_id(&permission.id)?;
		let record: Option<PermissionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete permission".into()),
			None => bail!("Failed to delete permission"),
		}
	}
}
