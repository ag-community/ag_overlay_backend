use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    ammo: i16,
    clip: i16,
    deaths: i16,
    frags: i16,
    health: i16,
    hev: i16,
    name: String,
    team: String,
    weapon: String,
}