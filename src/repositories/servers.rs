use std::{net::SocketAddr, ops::DerefMut, time::{SystemTime, UNIX_EPOCH}};

use redis::AsyncCommands;

use crate::{common::{context::{Context, PoolContext}, redis_json::Json}, models::server::{Server, ServerData, ServerPayload}};

const SERVER_KEY: &str = "ag_overlay:servers";
const SERVER_TIMESTAMPS_KEY: &str = "ag_overlay:server_timestamps";

pub async fn add_server<C: Context>(ctx: &C, addr: SocketAddr, server: ServerPayload) -> anyhow::Result<()> {
    let mut redis = ctx.redis().await?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    
    let _: () = redis::pipe().atomic()
        .hset(SERVER_KEY, addr.to_string(), Json(server))
        .hset(SERVER_TIMESTAMPS_KEY, addr.to_string(), timestamp)
        .query_async(redis.deref_mut()).await?;
    
    Ok(())
}

pub async fn list_all_servers<C: Context>(ctx: &C) -> anyhow::Result<Vec<Server>> {
    let mut redis = ctx.redis().await?;
    let servers: Vec<(String, Json<ServerData>)> = redis.hgetall(SERVER_KEY).await?;
    
    let servers = servers.into_iter()
        .map(|(addr, Json(server_data))| Server {
            server_ip: addr,
            data: server_data,
        })
        .collect();
    
    Ok(servers)
}

pub async fn remove_inactive_servers<C: Context>(ctx: &C, threshold_ms: u64) -> anyhow::Result<Vec<String>> {
    let mut redis = ctx.redis().await?;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    
    let timestamps: Vec<(String, u64)> = redis.hgetall(SERVER_TIMESTAMPS_KEY).await?;
    let mut removed_servers = Vec::new();
    
    for (addr, timestamp) in timestamps {
        if current_time - timestamp > threshold_ms {
            let _: () = redis::pipe().atomic()
                .hdel(SERVER_KEY, &addr)
                .hdel(SERVER_TIMESTAMPS_KEY, &addr)
                .query_async(redis.deref_mut()).await?;
                
            removed_servers.push(addr);
        }
    }
    
    Ok(removed_servers)
}