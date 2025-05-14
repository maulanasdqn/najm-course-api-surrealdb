use ::axum::{
	http::{header, HeaderValue, Method},
	response::Redirect,
	routing::get,
	Extension, Router,
};
use tower_http::cors::CorsLayer;
use utoipa_swagger_ui::SwaggerUi;

pub use najm_course_entities::*;
pub use najm_course_libs::*;
pub use najm_course_utils::*;

pub mod v1;
pub mod v2;

pub use v1::*;

pub async fn apps(
	surrealdb_ws: SurrealWsClient,
	surrealdb_mem: SurrealMemClient,
) -> Router {
	let state = AppState {
		surrealdb_ws,
		surrealdb_mem,
	};
	let env = Env::new();
	let cors_origins = match env.rust_env.as_str() {
		"development" => vec!["http://localhost:3000", "http://localhost:3002"],
		"production" => {
			vec![
				"https://cat.najmcourse.com",
				"https://v2.cat.najmcourse.com",
				"https://backoffice.najmcourse.com",
			]
		}
		_ => vec![
			"http://localhost:3000",
			"http://localhost:3002",
			"https://v2.cat.najmcourse.com",
			"https://cat.najmcourse.com",
			"https://backoffice.najmcourse.com",
		],
	};

	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	let cors_middleware = CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true);

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", v1::routes().await)
		.nest("/v2", v2::routes().await)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", v1::docs_router()))
		.layer(cors_middleware)
		.layer(Extension(state))
}
