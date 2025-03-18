use axum::{routing::post, Router};

pub mod gacha_controller;
pub mod gacha_dto;
pub mod gacha_repository;
pub mod gacha_schema;
pub mod gacha_service;

pub use gacha_dto::*;
pub use gacha_repository::*;
pub use gacha_schema::*;
pub use gacha_service::*;

pub fn gacha_router() -> Router {
	Router::new()
		.route(
			"/create/claim",
			post(gacha_controller::post_create_gacha_claim),
		)
		.route(
			"/create/item",
			post(gacha_controller::post_create_gacha_item),
		)
		.route(
			"/create/roll",
			post(gacha_controller::post_create_gacha_roll),
		)
}
