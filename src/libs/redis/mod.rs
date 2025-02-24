use redis::Client;
use redis::RedisResult;
use std::env;

pub async fn redisdb_init() -> RedisResult<Client> {
	let host_name =
		env::var("REDIS_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
	let uri_scheme = if env::var("IS_TLS").is_ok() {
		"rediss"
	} else {
		"redis"
	};
	let url = format!("{}://{}", uri_scheme, host_name);
	let client = Client::open(url)?;
	Ok(client)
}
