use deadpool_redis::{Config, Runtime};

pub fn create_redis_pool(connection_uri: String) -> deadpool_redis::Pool {

    dbg!(&connection_uri);
    let cfg = Config::from_url(connection_uri);


    let pool = cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    pool

}