use anyhow::{bail, Result};
use najm_course_entities::{AppState, MetaRequestDto, ResponseListSuccessDto};
use najm_course_libs::ResourceEnum;
use najm_course_utils::{get_id, get_iso_date, make_thing, query_list_with_meta};
use validator::Validate;

use super::{
	SessionsCreateRequestDto, SessionsDetailResponseDto, SessionsDetailSchema,
	SessionsResponseDto, SessionsSchema, SessionsUpdateRequestDto, TestSessionsSchema,
};

pub struct SessionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> SessionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_session_by_id(&self, id: &str) -> Result<SessionsSchema> {
		let db = &self.state.surrealdb_ws;
		let session: Option<SessionsSchema> =
			db.select((&ResourceEnum::Sessions.to_string(), id)).await?;
		match session {
			Some(s) if !s.is_deleted => Ok(s),
			_ => bail!("Session not found"),
		}
	}

	pub async fn query_session_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<SessionsResponseDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if let Some(_search) = &meta.search {
			conditions
				.push("string::contains(string::lowercase(name ?? ''), $search)".into());
		}
		if let (Some(filter_by), Some(_filter)) = (&meta.filter_by, &meta.filter) {
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw = query_list_with_meta::<SessionsSchema>(
			&self.state.surrealdb_ws,
			&ResourceEnum::Sessions.to_string(),
			&meta,
			conditions,
			None,
		)
		.await?;
		let data = raw
			.data
			.into_iter()
			.map(SessionsResponseDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: raw.meta,
		})
	}

	pub async fn query_session_by_id(
		&self,
		id: &str,
	) -> Result<SessionsDetailResponseDto> {
		let db = &self.state.surrealdb_ws;
		let query = format!(
			"SELECT * FROM {}:⟨{}⟩ FETCH tests.test, tests.test.questions, tests.test.questions.options",
			ResourceEnum::Sessions.to_string(),
			id
		);
		let mut result = db.query(query).await?;
		let session: Option<SessionsDetailSchema> = result.take(0)?;
		match session {
			Some(s) if !s.is_deleted => Ok(SessionsDetailResponseDto::from(s)),
			_ => bail!("Session not found"),
		}
	}

	pub async fn query_create_session(
		&self,
		payload: SessionsCreateRequestDto,
	) -> Result<String> {
		payload.validate()?;
		if payload.tests.is_empty() {
			bail!("Tests must not be empty");
		}
		let db = &self.state.surrealdb_ws;
		let session_id = surrealdb::Uuid::new_v4().to_string();
		let now = get_iso_date();
		let tests = payload
			.tests
			.into_iter()
			.map(|t| TestSessionsSchema {
				test: make_thing(&ResourceEnum::Tests.to_string(), &t.test_id),
				weight: t.weight,
				multiplier: t.multiplier,
				shuffle: t.shuffle,
				start_date: t.start_date,
				end_date: t.end_date,
			})
			.collect::<Vec<_>>();
		let session = SessionsSchema {
			id: make_thing(&ResourceEnum::Sessions.to_string(), &session_id),
			name: payload.name,
			category: payload.category,
			description: payload.description,
			student_type: payload.student_type,
			tests,
			is_active: payload.is_active,
			is_deleted: false,
			created_at: now.clone(),
			updated_at: now,
		};
		let _res: Option<SessionsSchema> = db
			.create((ResourceEnum::Sessions.to_string(), session_id.clone()))
			.content(session)
			.await?;
		Ok(session_id)
	}

	pub async fn query_update_session(
		&self,
		id: String,
		data: SessionsUpdateRequestDto,
	) -> Result<String> {
		data.validate()?;
		if data.tests.is_empty() {
			bail!("Tests must not be empty");
		}
		let db = &self.state.surrealdb_ws;
		let existing = self.query_raw_session_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Session already deleted");
		}
		let tests = data
			.tests
			.into_iter()
			.map(|t| TestSessionsSchema {
				test: make_thing(&ResourceEnum::Tests.to_string(), &t.test_id),
				weight: t.weight,
				shuffle: t.shuffle,
				multiplier: t.multiplier,
				start_date: t.start_date,
				end_date: t.end_date,
			})
			.collect::<Vec<_>>();
		let updated = SessionsSchema {
			id: make_thing(&ResourceEnum::Sessions.to_string(), &id),
			name: data.name,
			category: data.category,
			description: data.description,
			student_type: data.student_type,
			tests,
			is_active: data.is_active,
			is_deleted: false,
			created_at: existing.created_at,
			updated_at: get_iso_date(),
		};
		let record: Option<SessionsSchema> =
			db.update(get_id(&updated.id)?).content(updated).await?;
		match record {
			Some(_) => Ok("Success update session".into()),
			None => bail!("Failed to update session"),
		}
	}

	pub async fn query_delete_session(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let session_id = make_thing(&ResourceEnum::Sessions.to_string(), &id);
		let session = self
			.query_raw_session_by_id(&session_id.id.to_raw())
			.await?;
		if session.is_deleted {
			bail!("Session already deleted");
		}
		let record_key = get_id(&session_id)?;
		let record: Option<SessionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete session".into()),
			None => bail!("Failed to delete session"),
		}
	}
}
