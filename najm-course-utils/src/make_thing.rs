use surrealdb::sql::Thing;

pub fn make_thing(table: &str, id: &str) -> Thing {
	Thing::from((table, id))
}

#[cfg(test)]
mod make_thing_test {
	use surrealdb::sql::Id;

	use super::*;

	#[test]
	fn test_make_thing_should_return_thing() {
		let thing = make_thing("test", "123");
		assert_eq!(thing.tb, "test");
		assert_eq!(thing.id, Id::String("123".into()));
	}
}
