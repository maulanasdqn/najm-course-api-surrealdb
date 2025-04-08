use super::{
	QuestionsItemDto, QuestionsRequestDto, QuestionsResponseListDto, QuestionsSchema,
};
use crate::{
	get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto, ResourceEnum,
	ResponseListSuccessDto,
};
use anyhow::{bail, Result};
use najm_course_utils::get_iso_date;
use surrealdb::Uuid;

pub struct QuestionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> QuestionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_question_by_id(&self, id: &str) -> Result<QuestionsSchema> {
		let db = &self.state.surrealdb_ws;
		let question: Option<QuestionsSchema> =
			db.select((ResourceEnum::Questions.to_string(), id)).await?;
		match question {
			Some(q) if !q.is_deleted => Ok(q),
			_ => bail!("Question not found"),
		}
	}

	pub async fn query_question_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<QuestionsResponseListDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions
				.push("string::contains(string::lowercase(question ?? ''), $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw = query_list_with_meta::<QuestionsSchema>(
			&self.state.surrealdb_ws,
			&ResourceEnum::Questions.to_string(),
			&meta,
			conditions,
			None,
		)
		.await?;
		let transformed = raw
			.data
			.into_iter()
			.map(QuestionsResponseListDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data: transformed,
			meta: raw.meta,
		})
	}

	pub async fn query_question_by_id(&self, id: String) -> Result<QuestionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let query = format!(
			"SELECT * FROM app_questions:⟨{}⟩ WHERE is_deleted = false",
			id
		);
		let mut result = db.query(query).await?;
		let question: Option<QuestionsSchema> = result.take(0)?;
		let question = match question {
			Some(q) if !q.is_deleted => q,
			_ => bail!("Question not found"),
		};
		Ok(QuestionsItemDto::from(question))
	}

	pub async fn query_create_question(
		&self,
		payload: QuestionsRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let question_id = Uuid::new_v4().to_string();
		let question = QuestionsSchema {
			id: make_thing(&ResourceEnum::Questions.to_string(), &question_id),
			question: payload.question,
			discussion: payload.discussion,
			question_image_url: payload.question_image_url,
			discussion_image_url: payload.discussion_image_url,
			options: payload
				.options
				.iter()
				.map(|id| make_thing("app_options", id))
				.collect(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let _: Option<QuestionsSchema> = db
			.create((&ResourceEnum::Questions.to_string(), question_id))
			.content(question)
			.await?;
		Ok("Question created successfully".into())
	}

	pub async fn query_update_question(
		&self,
		id: String,
		data: QuestionsRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let thing_id = make_thing(&ResourceEnum::Questions.to_string(), &id);
		let existing = self.query_raw_question_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Question already deleted");
		}
		let merged = QuestionsSchema {
			id: thing_id,
			question: data.question,
			discussion: data.discussion,
			question_image_url: data.question_image_url,
			discussion_image_url: data.discussion_image_url,
			options: data
				.options
				.iter()
				.map(|id| make_thing("app_options", id))
				.collect(),
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			updated_at: get_iso_date(),
		};
		let record: Option<QuestionsSchema> =
			db.update(get_id(&merged.id)?).content(merged).await?;
		match record {
			Some(_) => Ok("Success update question".into()),
			None => bail!("Failed to update question"),
		}
	}

	pub async fn query_delete_question(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let question_id = make_thing(&ResourceEnum::Questions.to_string(), &id);
		let question = self
			.query_raw_question_by_id(&question_id.id.to_raw())
			.await?;
		if question.is_deleted {
			bail!("Question already deleted");
		}
		let record_key = get_id(&question_id)?;
		let record: Option<QuestionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete question".into()),
			None => bail!("Failed to delete question"),
		}
	}
}
