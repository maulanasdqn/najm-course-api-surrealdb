use env_logger::{init, Builder, Env};
use log::LevelFilter;
use najm_course_apis::{apps, axum_init};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
	let log_file = File::create("app.log").expect("Could not create log file");
	init();
	Builder::from_env(Env::default().default_filter_or("info"))
		.format(|buf, record| {
			writeln!(
				buf,
				"{} [{}] - {}",
				chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
				record.level(),
				record.args()
			)
		})
		.target(env_logger::Target::Pipe(Box::new(log_file)))
		.filter(None, LevelFilter::Info)
		.init();
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		apps(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
