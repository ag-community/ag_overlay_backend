use crate::{common::context::Context, repositories::servers};

pub async fn cleanup_servers<C: Context>(ctx: &C) -> anyhow::Result<()> {
    let removed_servers = servers::remove_inactive_servers(ctx, 2000).await?;
    if !removed_servers.is_empty() {
        info!("Removed inactive servers: {:?}", removed_servers);
    }
    Ok(())
}