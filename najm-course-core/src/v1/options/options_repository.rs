use super::{
	OptionsItemDto, OptionsRequestDto, OptionsResponseListDto, OptionsSchema,
};
use crate::{
	extract_id, get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	ResourceEnum, ResponseListSuccessDto,
};
use anyhow::{bail, Result};
use najm_course_utils::get_iso_date;
use surrealdb::Uuid;

pub struct OptionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> OptionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_option_by_id(&self, id: &str) -> Result<OptionsSchema> {
		let db = &self.state.surrealdb_ws;
		let role: Option<OptionsSchema> =
			db.select((ResourceEnum::Options.to_string(), id)).await?;
		match role {
			Some(r) if !r.is_deleted => Ok(r),
			_ => bail!("Options not found"),
		}
	}

	pub async fn query_option_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<OptionsResponseListDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions
				.push("string::contains(string::lowercase(label ?? ''), $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw = query_list_with_meta::<OptionsSchema>(
			&self.state.surrealdb_ws,
			&ResourceEnum::Options.to_string(),
			&meta,
			conditions,
			None,
		)
		.await?;
		let transformed = raw
			.data
			.into_iter()
			.map(OptionsResponseListDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data: transformed,
			meta: raw.meta,
		})
	}

	pub async fn query_option_by_label(
		&self,
		label: String,
	) -> Result<OptionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE label = $label AND is_deleted = false LIMIT 1",
			ResourceEnum::Options.to_string()
		);
		let mut result = db.query(sql).bind(("label", label.clone())).await?;
		let option: Option<OptionsSchema> = result.take(0)?;
		let option = match option {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Option not found"),
		};
		Ok(OptionsItemDto {
			id: extract_id(&option.id),
			label: option.label,
			image_url: option.image_url,
			created_at: option.created_at,
			updated_at: option.updated_at,
		})
	}

	pub async fn query_option_by_id(&self, id: String) -> Result<OptionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let query = format!(
			"SELECT * FROM app_options:⟨{}⟩ WHERE is_deleted = false",
			id
		);
		let mut result = db.query(query).await?;
		let option: Option<OptionsSchema> = result.take(0)?;
		let option = match option {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Option not found"),
		};
		Ok(OptionsItemDto {
			id: extract_id(&option.id),
			label: option.label,
			image_url: option.image_url,
			created_at: option.created_at,
			updated_at: option.updated_at,
		})
	}

	pub async fn query_create_option(
		&self,
		payload: OptionsRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let option_id = Uuid::new_v4().to_string();
		let option = OptionsSchema {
			id: make_thing(&ResourceEnum::Options.to_string(), &option_id),
			label: payload.label,
			is_deleted: false,
			is_correct: payload.is_correct,
			image_url: payload.image_url,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let _: Option<OptionsSchema> = db
			.create((&ResourceEnum::Options.to_string(), option_id))
			.content(option)
			.await?;
		Ok("Option created successfully".into())
	}

	pub async fn query_update_option(
		&self,
		id: String,
		data: OptionsRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let thing_id = make_thing(&ResourceEnum::Options.to_string(), &id);
		let existing = self.query_raw_option_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Option already deleted");
		}
		let merged = OptionsSchema {
			id: thing_id,
			label: data.label,
			image_url: data.image_url,
			is_correct: data.is_correct,
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			updated_at: get_iso_date(),
		};
		let record: Option<OptionsSchema> =
			db.update(get_id(&merged.id)?).content(merged).await?;
		match record {
			Some(_) => Ok("Success update option".into()),
			None => bail!("Failed to update option"),
		}
	}

	pub async fn query_delete_option(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let option_id = make_thing(&ResourceEnum::Options.to_string(), &id);
		let option = self.query_raw_option_by_id(&option_id.id.to_raw()).await?;
		if option.is_deleted {
			bail!("Option already deleted");
		}
		let record_key = get_id(&option_id)?;
		let record: Option<OptionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete option".into()),
			None => bail!("Failed to delete option"),
		}
	}
}
