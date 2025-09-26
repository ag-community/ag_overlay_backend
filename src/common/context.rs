use crate::common::redis_pool::{PoolResult, RedisPool};
use async_trait::async_trait;

pub trait Context: Sync + Send {
    fn redis_pool(&self) -> &RedisPool;
}

#[async_trait]
pub trait PoolContext: Sync + Send {
    async fn redis(&self) -> PoolResult;
}

#[async_trait]
impl<T: Context> PoolContext for T {
    async fn redis(&self) -> PoolResult {
        self.redis_pool().get().await
    }
}