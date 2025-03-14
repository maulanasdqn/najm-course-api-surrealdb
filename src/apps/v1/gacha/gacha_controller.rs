use super::{GachaRequestDto, GachaService};
use crate::{v1::GachaCreateItemRequestDto, AppState, MessageResponseDto};
use axum::{response::IntoResponse, Extension, Json};

#[utoipa::path(
    post,
    path = "/v1/gacha/create",
    request_body = GachaRequestDto,
    responses(
        (status = 200, description = "Create gacha successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha(
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/gacha/create/item",
    request_body = GachaCreateItemRequestDto,
    responses(
        (status = 200, description = "Create gacha item successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha item failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_item(
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaCreateItemRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha_item(payload, &state).await
}
