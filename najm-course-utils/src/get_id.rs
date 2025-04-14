use anyhow::{Result, bail};
use surrealdb::sql::Thing;

pub fn get_id(thing: &Thing) -> Result<(&str, &str)> {
	let table = thing.tb.as_str();
	let id = match &thing.id {
		surrealdb::sql::Id::String(s) => s.as_str(),
		_ => bail!("Unsupported ID type"),
	};
	Ok((table, id))
}

pub fn extract_id(thing: &Thing) -> String {
	let id = thing.id.to_raw();
	id
}

#[cfg(test)]
mod get_id_test {
	use super::*;
	use crate::make_thing;
	#[test]
	fn test_get_id_should_return_table_and_id() {
		let thing = make_thing("test", "123");
		let (table, id) = get_id(&thing).unwrap();
		assert_eq!(table, "test");
		assert_eq!(id, "123");
	}
}
