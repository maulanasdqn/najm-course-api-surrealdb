use log::LevelFilter;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};

pub async fn db_pgsql() -> DatabaseConnection {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(&url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info)
        .set_schema_search_path("public");
    match Database::connect(opt).await {
        Ok(connect) => connect,
        Err(error) => panic!("{}", error),
    }
}
