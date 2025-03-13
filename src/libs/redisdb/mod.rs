use super::Env;
use redis::Client;
use redis::RedisResult;

pub async fn redisdb_init() -> RedisResult<Client> {
	let env = Env::new();
	let host_name = env.redis_hostname;
	let url = format!("redis://{}", host_name);
	let client = Client::open(url)?;
	Ok(client)
}
