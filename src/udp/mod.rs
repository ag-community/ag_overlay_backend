use tokio::net::UdpSocket;

use crate::{lifecycle, models::server::ServerPayload, repositories::servers, settings::AppSettings};

pub async fn serve(settings: &AppSettings) -> anyhow::Result<()> {
    let state = lifecycle::initialize_state(settings).await?;
    let addr = format!("0.0.0.0:{}", settings.udp_server_port);
    let listener = UdpSocket::bind(&addr).await?;
    info!("Listening UDP server on: {}", listener.local_addr()?);

    let mut buf = [0; 8192];

    loop {
        let (len, addr) = listener.recv_from(&mut buf).await?;
        let data = String::from_utf8_lossy(&buf[..len]);
        let server: ServerPayload = serde_json::from_str(data.as_ref())?;

        servers::add_server(&state, addr, server).await?;
    }
}