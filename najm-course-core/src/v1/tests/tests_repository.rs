use super::{
	TestsCreateRequestDto, TestsItemDto, TestsResponseListDto, TestsSchema,
	TestsUpdateRequestDto,
};
use crate::{
	get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	OptionsItemDto, OptionsRepository, OptionsSchema, QuestionsItemDto,
	QuestionsSchema, ResourceEnum, ResponseListSuccessDto,
};
use anyhow::{bail, Result};
use najm_course_utils::get_iso_date;
use surrealdb::Uuid;

pub struct TestsRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> TestsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_test_by_id(&self, id: &str) -> Result<TestsSchema> {
		let db = &self.state.surrealdb_ws;
		let test: Option<TestsSchema> =
			db.select((ResourceEnum::Tests.to_string(), id)).await?;
		match test {
			Some(t) if !t.is_deleted => Ok(t),
			_ => bail!("Test not found"),
		}
	}

	pub async fn query_test_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TestsResponseListDto>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if let Some(_search) = &meta.search {
			conditions
				.push("string::contains(string::lowercase(name ?? ''), $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		let raw = query_list_with_meta::<TestsSchema>(
			&self.state.surrealdb_ws,
			&ResourceEnum::Tests.to_string(),
			&meta,
			conditions,
			None,
		)
		.await?;
		let transformed = raw
			.data
			.into_iter()
			.map(TestsResponseListDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data: transformed,
			meta: raw.meta,
		})
	}

	pub async fn query_test_by_id(&self, id: String) -> Result<TestsItemDto> {
		let db = &self.state.surrealdb_ws;
		let test: Option<TestsSchema> =
			db.select((ResourceEnum::Tests.to_string(), id)).await?;
		let test = match test {
			Some(t) if !t.is_deleted => t,
			_ => bail!("Test not found"),
		};
		let options_repo = OptionsRepository::new(self.state);
		let mut questions = vec![];
		for thing in &test.questions {
			let question: Option<QuestionsSchema> =
				db.select((thing.tb.as_str(), thing.id.to_string())).await?;
			if let Some(q) = question {
				if q.is_deleted {
					continue;
				}
				let mut option_items = Vec::new();
				for opt_thing in &q.options {
					if let Ok(opt) = options_repo
						.query_raw_option_by_id(&opt_thing.id.to_string())
						.await
					{
						option_items.push(OptionsItemDto {
							id: match &opt.id.id {
								surrealdb::sql::Id::String(s) => s.clone(),
								_ => "".to_string(),
							},
							label: opt.label,
							image_url: opt.image_url,
							created_at: opt.created_at,
							updated_at: opt.updated_at,
						});
					}
				}
				questions.push(QuestionsItemDto::from_with_options(q, option_items));
			}
		}
		Ok(TestsItemDto::from_with_questions(test, questions))
	}

	pub async fn query_create_test(
		&self,
		payload: TestsCreateRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let test_id = Uuid::new_v4().to_string();
		let mut question_things = Vec::new();
		for question in &payload.questions {
			let question_id = Uuid::new_v4().to_string();
			let question_thing = make_thing("app_questions", &question_id);
			let mut option_things = Vec::new();
			for option in &question.options {
				let option_id = Uuid::new_v4().to_string();
				let option_thing = make_thing("app_options", &option_id);
				let option_schema = OptionsSchema {
					id: option_thing.clone(),
					label: option.label.clone(),
					image_url: option.image_url.clone(),
					is_correct: option.is_correct,
					is_deleted: false,
					created_at: get_iso_date(),
					updated_at: get_iso_date(),
				};
				let _: Option<OptionsSchema> = db
					.create(("app_options", option_id))
					.content(option_schema)
					.await?;

				option_things.push(option_thing);
			}
			let question_schema = QuestionsSchema {
				id: question_thing.clone(),
				question: question.question.clone(),
				discussion: question.discussion.clone(),
				question_image_url: question.question_image_url.clone(),
				discussion_image_url: question.discussion_image_url.clone(),
				options: option_things,
				is_deleted: false,
				created_at: get_iso_date(),
				updated_at: get_iso_date(),
			};
			let _: Option<QuestionsSchema> = db
				.create(("app_questions", question_id))
				.content(question_schema)
				.await?;
			question_things.push(question_thing);
		}
		let test = TestsSchema {
			id: make_thing(&ResourceEnum::Tests.to_string(), &test_id),
			name: payload.name,
			questions: question_things,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let _: Option<TestsSchema> = db
			.create((&ResourceEnum::Tests.to_string(), test_id))
			.content(test)
			.await?;
		Ok("Test created successfully".into())
	}

	pub async fn query_update_test(
		&self,
		id: String,
		payload: TestsUpdateRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let test_thing_id = make_thing(&ResourceEnum::Tests.to_string(), &id);
		let existing = self.query_raw_test_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Test already deleted");
		}
		let mut question_things = Vec::new();
		for question in &payload.questions {
			let question_id = question.id.clone();
			let question_thing = make_thing("app_questions", &question_id);
			let mut option_things = Vec::new();
			for option in &question.options {
				let option_id = option.id.clone();
				let option_thing = make_thing("app_options", &option_id);
				let option_schema = OptionsSchema {
					id: option_thing.clone(),
					label: option.label.clone(),
					image_url: option.image_url.clone(),
					is_correct: option.is_correct,
					is_deleted: false,
					created_at: get_iso_date(),
					updated_at: get_iso_date(),
				};
				let _: Option<OptionsSchema> = db
					.update(get_id(&option_thing)?)
					.content(option_schema)
					.await?;
				option_things.push(option_thing);
			}
			let question_schema = QuestionsSchema {
				id: question_thing.clone(),
				question: question.question.clone(),
				discussion: question.discussion.clone(),
				question_image_url: question.question_image_url.clone(),
				discussion_image_url: question.discussion_image_url.clone(),
				options: option_things,
				is_deleted: false,
				created_at: get_iso_date(),
				updated_at: get_iso_date(),
			};
			let _: Option<QuestionsSchema> = db
				.update(get_id(&question_thing)?)
				.content(question_schema)
				.await?;
			question_things.push(question_thing);
		}
		let updated_test = TestsSchema {
			id: test_thing_id,
			name: payload.name,
			questions: question_things,
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			updated_at: get_iso_date(),
		};
		let result: Option<TestsSchema> = db
			.update(get_id(&updated_test.id)?)
			.content(updated_test)
			.await?;
		match result {
			Some(_) => Ok("Success update test".into()),
			None => bail!("Failed to update test"),
		}
	}

	pub async fn query_delete_test(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let test_id = make_thing(&ResourceEnum::Tests.to_string(), &id);
		let test = self.query_raw_test_by_id(&test_id.id.to_raw()).await?;
		if test.is_deleted {
			bail!("Test already deleted");
		}
		let record_key = get_id(&test_id)?;
		let record: Option<TestsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete test".into()),
			None => bail!("Failed to delete test"),
		}
	}
}
