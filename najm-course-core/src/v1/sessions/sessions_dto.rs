use super::{SessionsDetailSchema, SessionsSchema};
use crate::{OptionsItemDto, QuestionsItemDto, TestsItemDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct TestSessionsDto {
	#[schema(example = "uuid")]
	pub test_id: String,

	#[schema(example = 50)]
	pub weight: u32,

	#[schema(example = 1.25)]
	pub multiplier: f32,

	#[schema(example = "2025-05-01T00:00:00Z")]
	pub start_date: String,

	#[schema(example = "2025-05-31T23:59:59Z")]
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct SessionsCreateRequestDto {
	#[schema(example = "Tryout Saintek 2025")]
	pub name: String,

	#[schema(example = "Saintek")]
	pub category: String,

	#[schema(example = "Simulasi tryout saintek untuk persiapan UTBK 2025")]
	pub description: String,

	#[schema(example = "SMA")]
	pub student_type: String,

	#[schema(value_type = Vec<TestSessionsDto>)]
	pub tests: Vec<TestSessionsDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct SessionsUpdateRequestDto {
	#[schema(example = "session:xyz789")]
	pub id: String,

	#[schema(example = "Tryout Saintek 2025 - Update")]
	pub name: String,

	#[schema(example = "Saintek")]
	pub category: String,

	#[schema(example = "Updated deskripsi tryout saintek")]
	pub description: String,

	#[schema(example = "SMA")]
	pub student_type: String,

	#[schema(value_type = Vec<TestSessionsDto>)]
	pub tests: Vec<TestSessionsDto>,

	#[schema(example = true)]
	pub is_active: bool,

	#[schema(example = false)]
	pub is_deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestSessionsItemDto {
	pub test: TestsItemDto,
	pub weight: u32,
	pub multiplier: f32,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsResponseDto {
	pub id: String,
	pub name: String,
	pub category: String,
	pub description: String,
	pub student_type: String,
	pub tests_count: u32,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsDetailResponseDto {
	pub id: String,
	pub name: String,
	pub category: String,
	pub description: String,
	pub student_type: String,
	pub tests: Vec<TestSessionsItemDto>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<SessionsDetailSchema> for SessionsDetailResponseDto {
	fn from(value: SessionsDetailSchema) -> Self {
		let tests: Vec<TestSessionsItemDto> = value
			.tests
			.into_iter()
			.map(|t| {
				let test = t.test;

				let questions = test
					.questions
					.into_iter()
					.filter_map(|q_opt| {
						q_opt.map(|q| {
							let options = q
								.options
								.into_iter()
								.filter_map(|o_opt| {
									o_opt.map(|o| OptionsItemDto {
										id: o.id.id.to_raw(),
										label: o.label,
										is_correct: None,
										points: None,
										image_url: o.image_url,
										created_at: o.created_at,
										updated_at: o.updated_at,
									})
								})
								.collect();

							QuestionsItemDto {
								id: q.id.id.to_raw(),
								question: q.question,
								discussion: q.discussion,
								question_image_url: q.question_image_url,
								discussion_image_url: q.discussion_image_url,
								options,
								created_at: q.created_at,
								updated_at: q.updated_at,
							}
						})
					})
					.collect();

				let test_item = TestsItemDto {
					id: test.id.id.to_string(),
					name: test.name,
					questions,
					created_at: test.created_at,
					updated_at: test.updated_at,
				};

				TestSessionsItemDto {
					test: test_item,
					weight: t.weight,
					multiplier: t.multiplier,
					start_date: t.start_date,
					end_date: t.end_date,
				}
			})
			.collect();

		Self {
			id: value.id.id.to_raw(),
			name: value.name,
			category: value.category,
			description: value.description,
			student_type: value.student_type,
			tests,
			is_active: value.is_active,
			is_deleted: value.is_deleted,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl From<SessionsSchema> for SessionsResponseDto {
	fn from(value: SessionsSchema) -> Self {
		Self {
			id: value.id.id.to_raw(),
			name: value.name,
			category: value.category,
			description: value.description,
			student_type: value.student_type,
			tests_count: value.tests.len() as u32,
			is_active: value.is_active,
			is_deleted: value.is_deleted,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
