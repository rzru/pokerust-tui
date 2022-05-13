use serde::{Deserialize, Serialize};
use tui::text::Span;

use crate::utils::PreparePokemonNameForDisplay;

use super::{
    NamedApiResource, PokemonAbility, PokemonHeldItem, PokemonMove, PokemonSprites, PokemonStat,
    PokemonType, VersionGameIndex,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub abilities: Option<Vec<PokemonAbility>>,
    pub forms: Option<Vec<NamedApiResource>>,
    pub game_indices: Option<Vec<VersionGameIndex>>,
    pub held_items: Option<Vec<PokemonHeldItem>>,
    pub location_area_encounters: Option<String>,
    pub moves: Option<Vec<PokemonMove>>,
    pub sprites: Option<PokemonSprites>,
    pub species: Option<NamedApiResource>,
    pub stats: Option<Vec<PokemonStat>>,
    pub types: Option<Vec<PokemonType>>,
}

impl Pokemon {
    pub fn get_renderable_id(&self) -> Span {
        Span::raw(self.id.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_order(&self) -> Span {
        Span::raw(self.order.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_name(&self) -> Span {
        Span::raw(self.name.as_ref().unwrap().to_string().split_capitalize())
    }

    pub fn get_renderable_height(&self) -> Span {
        Span::raw(self.height.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_weight(&self) -> Span {
        Span::raw(self.weight.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_base_experience(&self) -> Span {
        Span::raw(self.base_experience.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_types(&self) -> Vec<Span> {
        self.types
            .as_ref()
            .unwrap()
            .iter()
            .map(|pokemon_type| pokemon_type.get_renderable())
            .collect()
    }
}
