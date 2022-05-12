use serde::{Deserialize, Serialize};

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub item: Option<NamedApiResource>,
    pub version_details: Option<Vec<PokemonHeldItemVersion>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItemVersion {
    pub rarity: Option<i32>,
    pub version: Option<NamedApiResource>,
}
