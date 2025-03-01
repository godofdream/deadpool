use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct Config {
    #[serde(default)]
    redis: deadpool_redis::Config,
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = ::config_crate::Config::default();
        cfg.merge(::config_crate::Environment::new().separator("__"))
            .unwrap();
        cfg.try_into().unwrap()
    }
}

fn create_pool() -> deadpool_redis::Pool {
    let cfg = Config::from_env();
    cfg.redis.create_pool().unwrap()
}

#[tokio::main]
#[test]
async fn test_pipeline() {
    use deadpool_redis::redis::pipe;
    let pool = create_pool();
    let mut conn = pool.get().await.unwrap();
    let (value,): (String,) = pipe()
        .cmd("SET")
        .arg("deadpool/pipeline_test_key")
        .arg("42")
        .ignore()
        .cmd("GET")
        .arg("deadpool/pipeline_test_key")
        .query_async(&mut conn)
        .await
        .unwrap();
    assert_eq!(value, "42".to_string());
}

#[tokio::main]
#[test]
async fn test_high_level_commands() {
    use deadpool_redis::redis::AsyncCommands;
    let pool = create_pool();
    let mut conn = pool.get().await.unwrap();
    let _: () = conn.set("deadpool/hlc_test_key", 42).await.unwrap();
    let value: isize = conn.get("deadpool/hlc_test_key").await.unwrap();
    assert_eq!(value, 42);
}
