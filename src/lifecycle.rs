use crate::common::redis_pool::{RedisPool, RedisPoolManager};
use crate::common::state::{AppState};
use crate::settings::AppSettings;
use deadpool::Runtime;
use redis::io::tcp::{socket2, TcpSettings};
use redis::{AsyncConnectionConfig, Commands};

pub fn initialize_logging(settings: &AppSettings) {
    tracing_subscriber::fmt()
        .with_max_level(settings.log_level)
        .json()
        .with_timer(tracing_subscriber::fmt::time())
        .with_level(true)
        .init();
}

pub async fn initialize_state(settings: &AppSettings) -> anyhow::Result<AppState> {
    let redis = initialize_redis(settings)?;
    info!("Connected to Redis at {}", settings.redis_url);
    Ok(AppState { redis })
}

pub fn initialize_redis(settings: &AppSettings) -> anyhow::Result<RedisPool> {
    let redis_client = redis::Client::open(settings.redis_url.as_str())?;
    let mut conn = redis_client.get_connection_with_timeout(settings.redis_wait_timeout)?;
    let _: () = conn.ping()?;
    let redis_cfg = AsyncConnectionConfig::new()
        .set_connection_timeout(settings.redis_connection_timeout)
        .set_response_timeout(settings.redis_response_timeout)
        .set_tcp_settings(TcpSettings::default().set_keepalive(socket2::TcpKeepalive::new()));

    let redis_manager = RedisPoolManager::new(redis_client, redis_cfg);
    let redis = RedisPool::builder(redis_manager)
        .max_size(settings.redis_max_connections)
        .wait_timeout(Some(settings.redis_wait_timeout))
        .runtime(Runtime::Tokio1)
        .build()?;
    Ok(redis)
}