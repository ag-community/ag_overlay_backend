use ag_overlay_backend::{lifecycle, settings::AppSettings, udp, websocket, workers::crons};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = AppSettings::get();
    lifecycle::initialize_logging(settings);

    match settings.app_component.as_str() {
        "cleanup_cron" => crons::cleanup_cron::serve(settings).await,
        "udp" => udp::serve(settings).await,
        "websocket" => websocket::serve(settings).await,
        _ => panic!("Unknown app component"),
    }
}