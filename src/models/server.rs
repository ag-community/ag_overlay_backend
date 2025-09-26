use serde::{Deserialize, Serialize};

use crate::models::player::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerPayload {
    #[serde(rename(deserialize = "effectiveTime"))]
    effective_time: u16,
    hostname: String,
    players: Vec<Player>,
    timelimit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerData {
    effective_time: u16,
    hostname: String,
    players: Vec<Player>,
    timelimit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub server_ip: String,
    pub data: ServerData,
}