use serde::{Deserialize, Serialize};

use super::{FlavorTextEntry, NamedApiResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub accuracy: Option<i32>,
    pub pp: Option<i32>,
    pub power: Option<i32>,
    #[serde(rename = "type")]
    pub pk_type: Option<NamedApiResource>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveVersion {
    pub move_learn_method: Option<NamedApiResource>,
    pub version_group: Option<NamedApiResource>,
    pub level_learned_at: Option<i32>,
}
