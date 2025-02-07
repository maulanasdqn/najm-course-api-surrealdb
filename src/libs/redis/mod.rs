use std::env;

use redis::Client;

pub fn db_redis() -> redis::Connection {
    let host_name = env::var("REDIS_HOSTNAME").unwrap_or("localhost".to_string());

    let uri_scheme = if env::var("IS_TLS").is_ok() {
        "rediss"
    } else {
        "redis"
    };

    let url = format!("{}://{}", uri_scheme, host_name);

    Client::open(url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}
