use najm_course_apis::Env;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionItem {
	pub label: String,
	pub is_correct: bool,
	pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionItem {
	pub id: Thing,
	pub question: String,
	pub question_image_url: String,
	pub discussion: String,
	pub discussion_image_url: String,
	pub options: Vec<OptionItem>,
	pub test_id: Thing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = Env::new();
	let db = Surreal::new::<Ws>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

	let test_id = Thing::from(("app_tests", "9df5e9ff-7ea7-446c-a354-d995a1f1b0a2"));

	let question_0 = QuestionItem {
		id: Thing::from(("app_questions", "0cf96582-cd77-46e9-ab37-e611d7395994")),
		question: "Question 1?".to_string().to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 1.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 1".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 1".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 1".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 1".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"0cf96582-cd77-46e9-ab37-e611d7395994",
	))
	.content(question_0)
	.await?;
	println!("✅ Inserted question 1");

	let question_1 = QuestionItem {
		id: Thing::from(("app_questions", "052f876c-bb71-4dd9-87b7-a8c38eccb90b")),
		question: "Question 2?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 2.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 2".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 2".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 2".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 2".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"052f876c-bb71-4dd9-87b7-a8c38eccb90b",
	))
	.content(question_1)
	.await?;
	println!("✅ Inserted question 2");

	let question_2 = QuestionItem {
		id: Thing::from(("app_questions", "f81a01fc-5cec-49af-9ad4-5d5db61ca635")),
		question: "Question 3?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 3.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 3".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 3".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 3".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 3".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"f81a01fc-5cec-49af-9ad4-5d5db61ca635",
	))
	.content(question_2)
	.await?;
	println!("✅ Inserted question 3");

	let question_3 = QuestionItem {
		id: Thing::from(("app_questions", "28d2e363-43b1-4144-8fa2-5fe7c727e523")),
		question: "Question 4?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 4.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 4".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 4".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 4".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 4".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"28d2e363-43b1-4144-8fa2-5fe7c727e523",
	))
	.content(question_3)
	.await?;
	println!("✅ Inserted question 4");

	let question_4 = QuestionItem {
		id: Thing::from(("app_questions", "8f0b9650-3625-4686-bc0b-ca651eaef575")),
		question: "Question 5?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 5.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 5".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 5".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 5".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 5".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"8f0b9650-3625-4686-bc0b-ca651eaef575",
	))
	.content(question_4)
	.await?;
	println!("✅ Inserted question 5");

	let question_5 = QuestionItem {
		id: Thing::from(("app_questions", "eb2515c6-fb74-421a-a023-4d4c70c36154")),
		question: "Question 6?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 6.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 6".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 6".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 6".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 6".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"eb2515c6-fb74-421a-a023-4d4c70c36154",
	))
	.content(question_5)
	.await?;
	println!("✅ Inserted question 6");

	let question_6 = QuestionItem {
		id: Thing::from(("app_questions", "2a1997a9-7eb3-434d-8be7-143e932e3de1")),
		question: "Question 7?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 7.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 7".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 7".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 7".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 7".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"2a1997a9-7eb3-434d-8be7-143e932e3de1",
	))
	.content(question_6)
	.await?;
	println!("✅ Inserted question 7");

	let question_7 = QuestionItem {
		id: Thing::from(("app_questions", "87cf5c99-c53e-4b16-8945-72ad7354366d")),
		question: "Question 8?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 8.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 8".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 8".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 8".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 8".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"87cf5c99-c53e-4b16-8945-72ad7354366d",
	))
	.content(question_7)
	.await?;
	println!("✅ Inserted question 8");

	let question_8 = QuestionItem {
		id: Thing::from(("app_questions", "12fef868-a240-4355-9fb2-3ac3f9550b01")),
		question: "Question 9?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 9.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 9".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 9".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 9".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 9".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"12fef868-a240-4355-9fb2-3ac3f9550b01",
	))
	.content(question_8)
	.await?;
	println!("✅ Inserted question 9");

	let question_9 = QuestionItem {
		id: Thing::from(("app_questions", "43d0fe23-6628-4e13-aa97-8e99a112e428")),
		question: "Question 10?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 10.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 10".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 10".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 10".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 10".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"43d0fe23-6628-4e13-aa97-8e99a112e428",
	))
	.content(question_9)
	.await?;
	println!("✅ Inserted question 10");

	let question_10 = QuestionItem {
		id: Thing::from(("app_questions", "5e8104c8-932b-459f-adc4-3d8b0f433517")),
		question: "Question 11?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 11.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 11".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 11".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 11".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 11".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"5e8104c8-932b-459f-adc4-3d8b0f433517",
	))
	.content(question_10)
	.await?;
	println!("✅ Inserted question 11");

	let question_11 = QuestionItem {
		id: Thing::from(("app_questions", "d0efb349-a1dc-488d-892d-785682229596")),
		question: "Question 12?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 12.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 12".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 12".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 12".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 12".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"d0efb349-a1dc-488d-892d-785682229596",
	))
	.content(question_11)
	.await?;
	println!("✅ Inserted question 12");

	let question_12 = QuestionItem {
		id: Thing::from(("app_questions", "472d232c-59c2-42ca-ab8a-c8f5a0a46dad")),
		question: "Question 13?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 13.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 13".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 13".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 13".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 13".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"472d232c-59c2-42ca-ab8a-c8f5a0a46dad",
	))
	.content(question_12)
	.await?;
	println!("✅ Inserted question 13");

	let question_13 = QuestionItem {
		id: Thing::from(("app_questions", "b645ee0f-6a13-42c0-b067-d0cf3a3ca22a")),
		question: "Question 14?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 14.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 14".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 14".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 14".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 14".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"b645ee0f-6a13-42c0-b067-d0cf3a3ca22a",
	))
	.content(question_13)
	.await?;
	println!("✅ Inserted question 14");

	let question_14 = QuestionItem {
		id: Thing::from(("app_questions", "b92b2b9a-1bbc-4f8c-95e9-b47d05706ce3")),
		question: "Question 15?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 15.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 15".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 15".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 15".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 15".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"b92b2b9a-1bbc-4f8c-95e9-b47d05706ce3",
	))
	.content(question_14)
	.await?;
	println!("✅ Inserted question 15");

	let question_15 = QuestionItem {
		id: Thing::from(("app_questions", "e697a920-e66c-4ef2-9ba7-c460db061d60")),
		question: "Question 16?".to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 16.".to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 16".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 16".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 16".to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 16".to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"e697a920-e66c-4ef2-9ba7-c460db061d60",
	))
	.content(question_15)
	.await?;
	println!("✅ Inserted question 16");

	let question_16 = QuestionItem {
		id: Thing::from(("app_questions", "60a6546f-ab51-4ea8-8677-500542cdbf5b")),
		question: "Question 17?".to_string().to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 17.".to_string().to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 17".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 17".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 17".to_string().to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 17".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"60a6546f-ab51-4ea8-8677-500542cdbf5b",
	))
	.content(question_16)
	.await?;
	println!("✅ Inserted question 17");

	let question_17 = QuestionItem {
		id: Thing::from(("app_questions", "f7b61e11-6952-43bb-b5f9-835738f53c30")),
		question: "Question 18?".to_string().to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 18.".to_string().to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 18".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 18".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 18".to_string().to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 18".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"f7b61e11-6952-43bb-b5f9-835738f53c30",
	))
	.content(question_17)
	.await?;
	println!("✅ Inserted question 18");

	let question_18 = QuestionItem {
		id: Thing::from(("app_questions", "ef5d7685-ee6a-4fdc-8e22-124b74ee2f29")),
		question: "Question 19?".to_string().to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 19.".to_string().to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 19".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 19".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 19".to_string().to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 19".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"ef5d7685-ee6a-4fdc-8e22-124b74ee2f29",
	))
	.content(question_18)
	.await?;
	println!("✅ Inserted question 19");

	let question_19 = QuestionItem {
		id: Thing::from(("app_questions", "e3ee3e1a-c09f-4d73-b5e2-7e3218add1a3")),
		question: "Question 20?".to_string().to_string(),
		question_image_url: "".into(),
		discussion: "Explanation for question 20.".to_string().to_string(),
		discussion_image_url: "".into(),
		options: vec![
			OptionItem {
				label: "Wrong A 20".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong B 20".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
			OptionItem {
				label: "Correct Answer 20".to_string().to_string(),
				is_correct: true,
				image_url: "".into(),
			},
			OptionItem {
				label: "Wrong C 20".to_string().to_string(),
				is_correct: false,
				image_url: "".into(),
			},
		],
		test_id: test_id.clone(),
	};
	db.create::<Option<QuestionItem>>((
		"app_questions",
		"e3ee3e1a-c09f-4d73-b5e2-7e3218add1a3",
	))
	.content(question_19)
	.await?;
	println!("✅ Inserted question 20");

	println!("✅ Semua questions berhasil disimpan ke SurrealDB!");
	Ok(())
}
