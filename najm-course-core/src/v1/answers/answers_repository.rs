use crate::{get_id, make_thing, AppState, MetaRequestDto, ResponseListSuccessDto};
use anyhow::{bail, Result};
use najm_course_libs::ResourceEnum;
use najm_course_utils::{get_iso_date, query_list_with_meta};
use validator::Validate;

use super::{
	AnswersCreateRequestDto, AnswersResponseDto, AnswersSchema,
	AnswersUpdateRequestDto,
};

pub struct AnswersRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> AnswersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<AnswersResponseDto>>> {
		let conditions = vec!["is_deleted = false".into()];
		let raw = query_list_with_meta::<AnswersSchema>(
			&self.state.surrealdb_ws,
			&ResourceEnum::Answers.to_string(),
			&meta,
			conditions,
			None,
		)
		.await?;
		let data = raw.data.into_iter().map(AnswersResponseDto::from).collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: raw.meta,
		})
	}

	pub async fn query_by_id(&self, id: &str) -> Result<AnswersResponseDto> {
		let db = &self.state.surrealdb_ws;
		let answer: Option<AnswersSchema> =
			db.select((ResourceEnum::Answers.to_string(), id)).await?;
		match answer {
			Some(data) if !data.is_deleted => Ok(data.into()),
			_ => bail!("Answer not found"),
		}
	}

	pub async fn query_create(
		&self,
		payload: AnswersCreateRequestDto,
	) -> Result<String> {
		payload.validate()?;
		let db = &self.state.surrealdb_ws;
		let id = surrealdb::Uuid::new_v4().to_string();
		let now = get_iso_date();
		let answer = AnswersSchema {
			id: make_thing(&ResourceEnum::Answers.to_string(), &id),
			user: make_thing(&ResourceEnum::Users.to_string(), &payload.user),
			test: make_thing(&ResourceEnum::Tests.to_string(), &payload.test),
			question: make_thing(&ResourceEnum::Questions.to_string(), &payload.question),
			option: make_thing(&ResourceEnum::Options.to_string(), &payload.option),
			is_correct: payload.is_correct,
			is_deleted: false,
			created_at: now.clone(),
			updated_at: now,
		};
		let _res: Option<AnswersSchema> = db
			.create((ResourceEnum::Answers.to_string(), &id))
			.content(answer)
			.await?;
		Ok(id)
	}

	pub async fn query_update(
		&self,
		id: String,
		payload: AnswersUpdateRequestDto,
	) -> Result<String> {
		payload.validate()?;
		let db = &self.state.surrealdb_ws;
		let existing = self.query_by_id(&id).await?;
		let updated = AnswersSchema {
			id: make_thing(&ResourceEnum::Answers.to_string(), &id),
			user: make_thing(&ResourceEnum::Users.to_string(), &existing.user),
			test: make_thing(&ResourceEnum::Tests.to_string(), &existing.test),
			question: make_thing(&ResourceEnum::Questions.to_string(), &existing.question),
			option: make_thing(&ResourceEnum::Options.to_string(), &payload.option),
			is_correct: payload.is_correct,
			is_deleted: false,
			created_at: existing.created_at,
			updated_at: get_iso_date(),
		};
		let updated_key = get_id(&updated.id)?;
		let _res: Option<AnswersSchema> =
			db.update(updated_key).content(updated).await?;
		Ok("Success update answer".into())
	}

	pub async fn query_delete(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let existing = self.query_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Answer already deleted");
		}
		let answer_thing = make_thing(&ResourceEnum::Answers.to_string(), &id);
		let record_key = get_id(&answer_thing)?;
		let res: Option<AnswersSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match res {
			Some(_) => Ok("Success delete answer".into()),
			None => bail!("Failed to delete answer"),
		}
	}
}
