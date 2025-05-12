use super::{FlagsItemDto, FlagsSchema};
use crate::{
	get_id, get_iso_date, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	ResourceEnum, ResponseListSuccessDto,
};
use anyhow::{bail, Result};

pub struct FlagsRepository<'a> {
	state: &'a AppState,
}

impl<'a> FlagsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_flag_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<FlagsItemDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions
				.push("string::contains(string::lowercase(name ?? ''), $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw_result: ResponseListSuccessDto<Vec<FlagsItemDto>> =
			query_list_with_meta(
				&self.state.surrealdb_ws,
				&ResourceEnum::Flags.to_string(),
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

	pub async fn query_flag_by_id(&self, id: String) -> Result<FlagsSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<FlagsSchema> = db
			.select((ResourceEnum::Flags.to_string(), id.clone()))
			.await?;
		match result {
			Some(flag) if !flag.is_deleted => Ok(flag),
			_ => bail!("Flag not found"),
		}
	}

	pub async fn query_flag_by_name(&self, name: String) -> Result<FlagsSchema> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE name = $name AND is_deleted = false LIMIT 1",
			ResourceEnum::Flags.to_string()
		);
		let result: Vec<FlagsSchema> =
			db.query(sql).bind(("name", name.clone())).await?.take(0)?;
		if let Some(flag) = result.into_iter().next() {
			Ok(flag.into())
		} else {
			bail!("Flag not found")
		}
	}

	pub async fn query_create_flag(&self, data: FlagsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<FlagsSchema> = db
			.create(ResourceEnum::Flags.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create Flag".into()),
			None => bail!("Failed to create Flag"),
		}
	}

	pub async fn query_update_flag(&self, data: FlagsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_flag_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Flag already deleted");
		}
		let merged = FlagsSchema {
			created_at: existing.created_at,
			updated_at: Some(get_iso_date()),
			..data.clone()
		};
		let record: Option<FlagsSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update Flag".into()),
			None => bail!("Failed to update Flag"),
		}
	}

	pub async fn query_delete_flag(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let flag_id = make_thing(&ResourceEnum::Flags.to_string(), &id);
		let flag = self.query_flag_by_id(flag_id.id.to_raw()).await?;
		if flag.is_deleted {
			bail!("Flag already deleted");
		}
		let record_key = get_id(&flag.id)?;
		let record: Option<FlagsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete Flag".into()),
			None => bail!("Failed to delete Flag"),
		}
	}
}
