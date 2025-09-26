pub mod tasks;

use crate::{cron_tasks, lifecycle, settings::AppSettings, workers::crons::cleanup_cron::tasks::cleanup_servers};
use crate::workers::crons::cleanup_cron::cleanup_servers::cleanup_servers;

pub async fn serve(settings: &AppSettings) -> anyhow::Result<()> {
    let ctx = lifecycle::initialize_state(settings).await?;
    cron_tasks! {
        &ctx,
        cleanup_servers,
    }
    Ok(())
}