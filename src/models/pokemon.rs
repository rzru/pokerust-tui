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
        self.id
            .as_ref()
            .and_then(|id| Some(Span::raw(id.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_order(&self) -> Span {
        self.order
            .as_ref()
            .and_then(|order| Some(Span::raw(order.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_name(&self) -> Span {
        self.name
            .as_ref()
            .and_then(|name| Some(Span::raw(name.to_string().split_capitalize())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_height(&self) -> Span {
        self.height
            .as_ref()
            .and_then(|height| Some(Span::raw(height.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_weight(&self) -> Span {
        self.weight
            .as_ref()
            .and_then(|weight| Some(Span::raw(weight.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_base_experience(&self) -> Span {
        self.base_experience
            .as_ref()
            .and_then(|base_experience| Some(Span::raw(base_experience.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_types(&self) -> Vec<Span> {
        self.types
            .as_ref()
            .and_then(|types| {
                Some(
                    types
                        .iter()
                        .map(|pokemon_type| pokemon_type.get_renderable())
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::Span,
    };

    use crate::models::{NamedApiResource, PokemonType};

    use super::Pokemon;

    fn get_stub_pokemon() -> Pokemon {
        Pokemon {
            id: Some(1),
            name: Some(String::from("raichu")),
            base_experience: Some(200),
            height: Some(10),
            is_default: None,
            order: Some(1),
            weight: Some(100),
            abilities: None,
            forms: None,
            game_indices: None,
            held_items: None,
            location_area_encounters: None,
            moves: None,
            sprites: None,
            species: None,
            stats: None,
            types: Some(vec![PokemonType {
                slot: None,
                de_type: Some(NamedApiResource {
                    name: Some(String::from("electric")),
                    url: None,
                }),
            }]),
        }
    }

    #[test]
    fn pokemon_get_renderable_id() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_id(), Span::raw("1"))
    }

    #[test]
    fn pokemon_get_renderable_order() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_order(), Span::raw("1"))
    }

    #[test]
    fn pokemon_get_renderable_name() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_name(), Span::raw("Raichu"))
    }

    #[test]
    fn pokemon_get_renderable_height() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_height(), Span::raw("10"))
    }

    #[test]
    fn pokemon_get_renderable_weight() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_weight(), Span::raw("100"))
    }

    #[test]
    fn pokemon_get_renderable_base_experience() {
        let pokemon = get_stub_pokemon();
        assert_eq!(pokemon.get_renderable_base_experience(), Span::raw("200"))
    }

    #[test]
    fn pokemon_get_renderable_types() {
        let pokemon = get_stub_pokemon();
        assert_eq!(
            pokemon.get_renderable_types(),
            vec![Span::styled(
                "Electric ",
                Style::default().fg(Color::Rgb(255, 204, 51)),
            )]
        )
    }
}
