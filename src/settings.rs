use tracing::Level;

use crate::common::env::FromEnv;
use std::env;
use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Duration;

pub struct AppSettings {
    pub app_component: String,

    pub log_level: Level,

    pub websocket_port: u16,
    pub udp_server_port: u16,

    pub redis_url: String,
    pub redis_max_connections: usize,
    pub redis_connection_timeout: Duration,
    pub redis_response_timeout: Duration,
    pub redis_wait_timeout: Duration,
}

impl AppSettings {
    pub fn load_from_env() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let app_component = env::var("APP_COMPONENT")?;

        let log_level = Level::from_env("LOG_LEVEL")?;

        let websocket_port = u16::from_env("WEBSOCKET_PORT")?;
        let udp_server_port = u16::from_env("UDP_SERVER_PORT")?;

        let redis_url = env::var("REDIS_URL")?;
        let redis_max_connections = usize::from_env("REDIS_MAX_CONNECTIONS")?;
        let redis_connection_timeout_secs = u64::from_env("REDIS_CONNECTION_TIMEOUT_SECS")?;
        let redis_connection_timeout = Duration::from_secs(redis_connection_timeout_secs);
        let redis_response_timeout_secs = u64::from_env("REDIS_RESPONSE_TIMEOUT_SECS")?;
        let redis_response_timeout = Duration::from_secs(redis_response_timeout_secs);
        let redis_wait_timeout_secs = u64::from_env("REDIS_WAIT_TIMEOUT_SECS")?;
        let redis_wait_timeout = Duration::from_secs(redis_wait_timeout_secs);

        Ok(AppSettings {
            app_component,

            log_level,

            websocket_port,
            udp_server_port,

            redis_url,
            redis_max_connections,
            redis_connection_timeout,
            redis_response_timeout,
            redis_wait_timeout,
        })
    }

    pub fn get() -> &'static AppSettings {
        settings()
    }
}

pub fn settings() -> &'static AppSettings {
    static SETTINGS: LazyLock<AppSettings> =
        LazyLock::new(|| AppSettings::load_from_env().expect("Failed to load settings"));
    SETTINGS.deref()
}