use serde::{Deserialize, Serialize};

use super::{APIResource, FlavorTextEntry, NamedApiResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSpecies {
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<i32>,
    pub color: Option<NamedApiResource>,
    pub base_happiness: Option<i32>,
    pub is_legendary: Option<bool>,
    pub evolution_chain: Option<APIResource>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}
