use serde::{Deserialize, Serialize};

use super::{FlavorTextEntry, NamedApiResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: Option<bool>,
    pub slot: Option<i32>,
    pub ability: Option<NamedApiResource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbilityExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub effect_entries: Option<Vec<VerboseEffect>>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerboseEffect {
    pub effect: Option<String>,
    pub short_effect: Option<String>,
    pub language: Option<NamedApiResource>,
}
