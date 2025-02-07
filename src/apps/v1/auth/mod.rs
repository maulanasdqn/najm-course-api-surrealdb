use axum::{routing::post, Router};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_middleware;
pub mod auth_repository;

pub use auth_dto::*;
pub use auth_repository::*;

pub fn auth_router() -> Router {
	Router::new().route("/login", post(auth_controller::post_login))
}
