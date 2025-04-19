use std::vec;

use super::{
	AnswersCreateRequestDto, AnswersSchema, QuestionsItemAnswersDto,
	TestsItemAnswersDto,
};
use crate::{AppState, OptionsSchema, QuestionsRepository, TestsRepository};
use anyhow::{bail, Result};
use najm_course_libs::ResourceEnum;
use najm_course_utils::{get_id, get_iso_date, make_thing};
use validator::Validate;

pub struct AnswersRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> AnswersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_answer_by_id(&self, id: &str) -> Result<AnswersSchema> {
		let db = &self.state.surrealdb_ws;
		let answer: Option<AnswersSchema> =
			db.select((ResourceEnum::Answers.to_string(), id)).await?;
		match answer {
			Some(a) if !a.is_deleted => Ok(a),
			_ => bail!("Answer not found"),
		}
	}

	pub async fn query_test_with_answers(
		&self,
		test_id: &str,
		user_id: &str,
	) -> Result<TestsItemAnswersDto> {
		let db = &self.state.surrealdb_ws;
		let test_repo = TestsRepository::new(&self.state);
		let question_repo = QuestionsRepository::new(&self.state);

		let test = test_repo.query_test_by_id(test_id).await?;

		let answers: Vec<AnswersSchema> = db
			.query(&format!(
				"SELECT * FROM app_answers WHERE test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ AND is_deleted = false",
				test_id, user_id
			))
			.await?
			.take(0)?;

		let mut questions_dto = vec![];
		for answer in answers {
			let question_id = answer.question.id.to_raw();
			let _selected_option_id = answer.option.id.to_raw();
			let question = question_repo.query_question_by_id(&question_id).await?;
			let _options = question.options.clone();

			let options_dto = vec![];

			questions_dto.push(QuestionsItemAnswersDto {
				id: question.id,
				question: question.question,
				discussion: question.discussion,
				question_image_url: question.question_image_url,
				discussion_image_url: question.discussion_image_url,
				options: options_dto,
				created_at: question.created_at,
				updated_at: question.updated_at,
			});
		}

		Ok(TestsItemAnswersDto {
			id: test.id,
			name: test.name,
			questions: questions_dto,
			created_at: test.created_at,
			updated_at: test.updated_at,
		})
	}

	pub async fn query_by_id(&self, id: &str) -> Result<TestsItemAnswersDto> {
		let db = &self.state.surrealdb_ws;
		let answer: Option<AnswersSchema> =
			db.select((ResourceEnum::Answers.to_string(), id)).await?;
		let Some(answer) = answer else {
			bail!("Answer not found");
		};
		if answer.is_deleted {
			bail!("Answer not found");
		}
		dbg!(&answer);
		let user_id = answer.user.id.to_raw();
		let test_id = answer.test.id.to_raw();
		self.query_test_with_answers(&test_id, &user_id).await
	}

	pub async fn query_create(
		&self,
		payload: AnswersCreateRequestDto,
	) -> Result<TestsItemAnswersDto> {
		payload.clone().validate()?;
		let db = &self.state.surrealdb_ws;
		let test_repo = TestsRepository::new(&self.state);
		let question_repo = QuestionsRepository::new(&self.state);
		let now = get_iso_date();
		for entry in &payload.answers {
			let id = surrealdb::Uuid::new_v4().to_string();
			let selected_option: Option<OptionsSchema> = db
				.select((ResourceEnum::Options.to_string(), &entry.option_id))
				.await?;
			let is_correct = selected_option.map_or(false, |opt| opt.is_correct);
			let answer = AnswersSchema {
				id: make_thing(&ResourceEnum::Answers.to_string(), &id),
				user: make_thing(&ResourceEnum::Users.to_string(), &payload.user_id),
				test: make_thing(&ResourceEnum::Tests.to_string(), &payload.test_id),
				question: make_thing(
					&ResourceEnum::Questions.to_string(),
					&entry.question_id,
				),
				option: make_thing(&ResourceEnum::Options.to_string(), &entry.option_id),
				is_correct,
				is_deleted: false,
				created_at: now.clone(),
				updated_at: now.clone(),
			};
			let _: Option<AnswersSchema> = db
				.create((ResourceEnum::Answers.to_string(), &id))
				.content(answer)
				.await?;
		}
		let test_data = test_repo.query_test_by_id(&payload.test_id).await?;
		let answers: Vec<AnswersSchema> = db
			.query(&format!(
				"SELECT * FROM app_answers WHERE test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ AND is_deleted = false",
				&payload.test_id, &payload.user_id
			))
			.await?
			.take(0)?;
		let mut questions_dto = vec![];
		for answer in answers {
			let question_id = answer.question.id.to_raw();
			let _selected_option_id = answer.option.id.to_raw();
			let question = question_repo.query_question_by_id(&question_id).await?;
			let _options = question.options.clone();
			let options_converted = vec![];
			questions_dto.push(QuestionsItemAnswersDto {
				id: question.id,
				question: question.question,
				discussion: question.discussion,
				question_image_url: question.question_image_url,
				discussion_image_url: question.discussion_image_url,
				options: options_converted,
				created_at: question.created_at,
				updated_at: question.updated_at,
			});
		}
		Ok(TestsItemAnswersDto {
			id: test_data.id,
			name: test_data.name,
			questions: questions_dto,
			created_at: test_data.created_at,
			updated_at: test_data.updated_at,
		})
	}

	pub async fn query_delete(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let raw: Option<AnswersSchema> =
			db.select((ResourceEnum::Answers.to_string(), &id)).await?;
		let Some(answer) = raw else {
			bail!("Answer not found");
		};
		if answer.is_deleted {
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
