use crate::common::context::Context;
use crate::common::redis_pool::RedisPool;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisPool,
}

impl AppState {
    pub fn new(redis: RedisPool) -> Self {
        Self { redis }
    }

    pub fn from_ctx<C: Context>(ctx: &C) -> Self {
        Self {
            redis: ctx.redis_pool().clone(),
        }
    }
}

impl Context for AppState {
    fn redis_pool(&self) -> &RedisPool {
        &self.redis
    }
}