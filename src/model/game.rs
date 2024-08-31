use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::all::UserId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TownNight {
    pub day: DAY,
    pub players: Vec<Player>,
    pub votes: HashMap<UserId,i32>,
    pub skip: i32
}

#[derive(Clone, Debug, Deserialize, Serialize,PartialEq, Eq)]
pub enum DAY {
    NIGHT,
    MORNING,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ROLE {
    MEDIC,
    MURDERER,
    CARTOMANTE,
    VILLAGER,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: UserId,
    pub name: String,
    pub role: ROLE,
    pub action: bool,
    pub is_death: bool,
}

impl Player {
    pub fn new(id: UserId, name: String) -> Self {
        Self {
            id,
            name,
            role: ROLE::VILLAGER,
            action: false,
            is_death: false,
        }
    }
}

impl TownNight {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            day: DAY::MORNING,
            players,
            votes: HashMap::new(),
            skip: 0
        }
    }
}

impl std::fmt::Display for ROLE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
