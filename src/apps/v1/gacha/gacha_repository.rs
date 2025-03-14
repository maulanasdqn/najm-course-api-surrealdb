use super::{
	GachaCreateItemRequestDto, GachaItemSchema, GachaRequestDto, GachaResponseDto,
	GachaSchema,
};
use crate::{v1::AuthRepository, AppState, ResourceEnum};
use anyhow::{bail, Result};
use surrealdb::sql::{Id, Thing};

pub struct GachaRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> GachaRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_by_transaction_number(
		&self,
		transaction_number: String,
	) -> Result<GachaResponseDto> {
		let db = &self.state.surrealdb;

		let result = db
			.select((ResourceEnum::Gacha.to_string(), transaction_number))
			.await?;

		match result {
			Some(response) => Ok(response),
			None => bail!("Gacha not found"),
		}
	}

	pub async fn query_create_gacha(&self, data: GachaRequestDto) -> Result<String> {
		let auth_repository = AuthRepository::new(self.state);
		let db = &self.state.surrealdb;

		let user = auth_repository
			.query_user_by_email(data.email.clone())
			.await?;

		let user_thing =
			Thing::from((ResourceEnum::Users.to_string(), Id::String(user.email)));

		let record: Option<GachaSchema> = db
			.create((
				ResourceEnum::GachaClaims.to_string(),
				&data.transaction_number,
			))
			.content(GachaSchema {
				transaction_number: data.transaction_number.clone(),
				user: user_thing,
			})
			.await?;

		match record {
			Some(_) => Ok("Gacha successfully created".to_string()),
			None => bail!("Failed to create gacha record"),
		}
	}

	pub async fn query_create_gacha_item(
		&self,
		data: GachaCreateItemRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb;

		let record: Option<GachaItemSchema> = db
			.create((ResourceEnum::Gacha.to_string(), data.item_name.clone()))
			.content(GachaItemSchema {
				item_name: data.item_name.clone(),
				item_image: data.item_image.clone(),
			})
			.await?;

		match record {
			Some(_) => Ok("Gacha item successfully created".to_string()),
			None => bail!("Failed to create gacha item record"),
		}
	}
}
