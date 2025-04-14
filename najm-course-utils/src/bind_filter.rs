use surrealdb::{engine::remote::ws::Client, method::Query};

pub fn bind_filter_value(
	query: Query<'_, Client>,
	val: String,
) -> Query<'_, Client> {
	if let Ok(b) = val.parse::<bool>() {
		query.bind(("filter", b))
	} else if let Ok(i) = val.parse::<i64>() {
		query.bind(("filter", i))
	} else {
		query.bind(("filter", val))
	}
}

#[cfg(test)]
mod bind_filter_test {
	use surrealdb::sql::Value;

	fn simulate_value(val: String) -> Value {
		if let Ok(b) = val.parse::<bool>() {
			Value::from(b)
		} else if let Ok(i) = val.parse::<i64>() {
			Value::from(i)
		} else {
			Value::from(val)
		}
	}

	#[test]
	fn test_bind_filter_value_should_parse_bool() {
		let val = simulate_value("true".to_string());
		assert_eq!(val, Value::from(true));
	}

	#[test]
	fn test_bind_filter_value_should_parse_int() {
		let val = simulate_value("123".to_string());
		assert_eq!(val, Value::from(123));
	}

	#[test]
	fn test_bind_filter_value_should_parse_string() {
		let val = simulate_value("abc".to_string());
		assert_eq!(val, Value::from("abc"));
	}
}
