use chrono::{DateTime, Utc};

pub fn get_iso_date() -> String {
	let now: DateTime<Utc> = Utc::now();
	now.to_rfc3339()
}

#[cfg(test)]
mod generate_date_test {
	use super::*;

	#[test]
	fn test_get_iso_date_should_return_iso_date() {
		let date = get_iso_date();
		assert!(!date.is_empty());
	}
}
