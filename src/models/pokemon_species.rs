use serde::{Deserialize, Serialize};

use super::{APIResource, FlavorTextEntry};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSpecies {
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<i32>,
    pub base_happiness: Option<i32>,
    pub is_legendary: Option<bool>,
    pub evolution_chain: Option<APIResource>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}
