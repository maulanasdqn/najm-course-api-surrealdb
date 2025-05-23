use std::vec;

use super::{
	AnswersCreateRequestDto, AnswersSchema, OptionsItemAnswersDto,
	QuestionsItemAnswersDto, TestsItemAnswersDto,
};
use crate::{
	AppState, OptionsSchema, QuestionsRepository, SessionsRepository, TestsRepository,
};
use anyhow::{bail, Error, Result};
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
		session_id: &str,
		test_id: &str,
		user_id: &str,
	) -> Result<TestsItemAnswersDto> {
		let db = &self.state.surrealdb_ws;
		let question_repo = QuestionsRepository::new(&self.state);
		let session_repo = SessionsRepository::new(&self.state);

		let session = session_repo.query_session_by_id(session_id).await?;
		let test = session
			.tests
			.into_iter()
			.find(|t| t.test.id == test_id)
			.ok_or_else(|| Error::msg("Test not found in session"))?;

		let answers: Vec<AnswersSchema> = db
			.query(&format!(
				"SELECT * FROM app_answers WHERE test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ AND session = app_sessions:⟨{}⟩ AND is_deleted = false",
				test_id, user_id, session_id
			))
			.await?
			.take(0)?;

		let answer_id = answers
			.get(0)
			.ok_or_else(|| Error::msg("No answers found"))?
			.id
			.id
			.to_raw()
			.clone();

		let mut questions_dto = Vec::new();

		for answer in &answers {
			let question_id = answer.question.id.to_raw();
			let selected_option_id = answer.option.id.to_raw();
			let question = question_repo.query_question_by_id(&question_id).await?;

			let options_dto = question
				.options
				.iter()
				.map(|opt| OptionsItemAnswersDto {
					id: opt.id.clone(),
					label: opt.label.clone(),
					is_user_selected: opt.id == selected_option_id,
					points: opt.points,
					is_correct: opt.is_correct.unwrap_or(false),
					image_url: opt.image_url.clone(),
					created_at: opt.created_at.clone(),
					updated_at: opt.updated_at.clone(),
				})
				.collect();

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

		let test_response = test.clone().test;
		let mut score = 0;

		if session.category == "Akademik" {
			let correct_count = questions_dto
				.iter()
				.filter(|q| q.options.iter().any(|o| o.is_user_selected && o.is_correct))
				.count();
			let raw_score = correct_count as f64 * test.multiplier as f64;
			score = (test.weight as f64 * raw_score).round() as i32;
		}

		if session.category == "Psikologi" {
			let test_name = test.clone().test.name.to_lowercase();

			let is_kepribadian = test_name.contains("kepribadian");
			let is_kecerdasan = test_name.contains("kecerdasan");
			let is_kecermatan = test_name.contains("kecermatan");

			let total_points: i32 = questions_dto
				.iter()
				.flat_map(|q| &q.options)
				.filter(|o| o.is_user_selected)
				.map(|o| o.points.unwrap_or(0))
				.sum();

			if is_kepribadian {
				score = ((total_points as f64) * 100.0 / 500.0).round() as i32;
			}

			if is_kecerdasan {
				score = total_points;
			}

			if is_kecermatan {
				// TODO: Implement scoring logic for kecermatan
			}
		}

		Ok(TestsItemAnswersDto {
			id: answer_id,
			name: test_response.name,
			score,
			questions: questions_dto,
			created_at: test_response.created_at,
			updated_at: test_response.updated_at,
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
		let session_id = answer.session.id.to_raw();
		self
			.query_test_with_answers(&session_id, &test_id, &user_id)
			.await
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
				session: make_thing(
					&ResourceEnum::Sessions.to_string(),
					&payload.session_id,
				),
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
				"SELECT * FROM app_answers WHERE session = app_sessions:⟨{}⟩ AND test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ AND is_deleted = false",
				&payload.session_id, &payload.test_id, &payload.user_id
			))
			.await?
			.take(0)?;
		let answer_id = answers
			.get(0)
			.ok_or_else(|| Error::msg("No answers found"))?
			.id
			.id
			.to_raw()
			.clone();
		let mut questions_dto = vec![];
		for answer in &answers {
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
			id: answer_id,
			name: test_data.name,
			score: 0,
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
