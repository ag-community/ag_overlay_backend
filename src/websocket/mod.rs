use std::{net::SocketAddr, time::Duration};

use crate::{common::context::Context, lifecycle, repositories::servers, settings::AppSettings};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::{Error, Message}};

pub async fn serve(settings: &AppSettings) -> anyhow::Result<()> {
    let state = lifecycle::initialize_state(settings).await?;
    let addr = format!("0.0.0.0:{}", settings.websocket_port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening websocket server on: {}", listener.local_addr()?);

    while let std::result::Result::Ok((stream, _)) = listener.accept().await {
        let task_state= state.clone();
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", addr);
        tokio::spawn(async move {
            let ctx = task_state;
            accept_connection(&ctx, peer, stream).await;
        });
    }

    anyhow::Ok(())
}

async fn accept_connection<C: Context>(ctx: &C, peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(ctx, peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection<C: Context>(ctx: &C, peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let ws_stream = accept_async(stream).await?;
    info!("New WebSocket connection: {}", peer);

    let (mut ws_sender, _) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match servers::list_all_servers(ctx).await {
                    Ok(servers) => {
                        let payload = serde_json::to_string(&servers).unwrap_or("[]".to_string());
                        ws_sender.send(Message::text(payload)).await?;
                    },
                    Err(err) => {
                        error!("Failed to list servers: {}", err);
                        ws_sender.send(Message::text("[]")).await?;
                    }
                }
            }
        }
    }
}
