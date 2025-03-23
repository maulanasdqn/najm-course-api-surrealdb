use crate::{CountResult, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto};
use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use super::bind_filter_value;

pub async fn query_list_with_meta<T>(
	db: &Surreal<Client>,
	table: &str,
	meta: &MetaRequestDto,
	conditions: Vec<String>,
) -> Result<ResponseListSuccessDto<Vec<T>>>
where
	T: DeserializeOwned + Serialize,
{
	let page = meta.page.unwrap_or(1);
	let per_page = meta.per_page.unwrap_or(10);
	if page < 1 || per_page < 1 {
		bail!("Invalid pagination: page and per_page must be greater than 0");
	}
	let start = (page - 1) * per_page;
	let mut sql = format!("SELECT * FROM {}", table);
	if !conditions.is_empty() {
		sql.push_str(" WHERE ");
		sql.push_str(&conditions.join(" AND "));
	}
	if let Some(sort_by) = &meta.sort_by {
		let order = match meta
			.order
			.clone()
			.unwrap_or_else(|| "ASC".into())
			.to_uppercase()
			.as_str()
		{
			"ASC" => "ASC",
			"DESC" => "DESC",
			_ => "ASC",
		};
		sql.push_str(&format!(" ORDER BY {} {}", sort_by, order));
	}
	sql.push_str(" LIMIT $per_page START $start");
	let mut query = db.query(sql);
	if let Some(search) = &meta.search {
		if !search.is_empty() {
			query = query.bind(("search", search.clone()));
		}
	}
	if let Some(filter_val) = meta.filter.clone() {
		query = bind_filter_value(query, filter_val);
	}
	query = query.bind(("per_page", per_page)).bind(("start", start));
	let items: Vec<T> = query.await?.take(0)?;
	let mut count_sql = format!("SELECT count() FROM {}", table);
	if !conditions.is_empty() {
		count_sql.push_str(" WHERE ");
		count_sql.push_str(&conditions.join(" AND "));
	}
	let mut count_query = db.query(count_sql);
	if let Some(filter_val) = meta.filter.clone() {
		count_query = bind_filter_value(count_query, filter_val);
	}
	let count_result: Vec<CountResult> = count_query.await?.take(0)?;
	let total = count_result.first().map(|c| c.count);
	let meta = MetaResponseDto {
		page: Some(page),
		per_page: Some(per_page),
		total,
	};
	Ok(ResponseListSuccessDto {
		data: items,
		meta: Some(meta),
	})
}
