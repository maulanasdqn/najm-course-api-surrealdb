use super::{GachaClaimRequestDto, GachaService};
use crate::{v1::GachaCreateItemRequestDto, AppState, MessageResponseDto};
use axum::{http::HeaderMap, response::IntoResponse, Extension, Json};

#[utoipa::path(
    post,
    path = "/v1/gacha/create/claims",
    request_body = GachaClaimRequestDto,
    responses(
        (status = 200, description = "Create gacha claims successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha claims failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_claims(
	header: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaClaimRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha_claims(payload, &state, header).await
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
