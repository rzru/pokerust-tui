use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tui::{text::Span, widgets::Row};

use crate::utils::PrepareForDisplay;

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
                        .par_iter()
                        .map(|pokemon_type| pokemon_type.get_renderable())
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }

    pub fn get_renderable_stats(&self) -> Vec<Row> {
        self.stats
            .as_ref()
            .and_then(|stats| {
                Some(
                    stats
                        .par_iter()
                        .map(|stat| stat.get_renderable_as_row())
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }

    pub fn get_renderable_held_items(&self, selected_version: &str) -> Vec<Row> {
        self.held_items
            .as_ref()
            .and_then(|held_items| {
                let mut prepared_held_items: Vec<Row> = vec![];

                held_items.iter().for_each(|held_item| {
                    prepared_held_items.extend(held_item.get_renderable_as_rows(selected_version))
                });

                Some(prepared_held_items)
            })
            .unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::Span,
        widgets::Row,
    };

    use crate::models::{
        NamedApiResource, PokemonHeldItem, PokemonHeldItemVersion, PokemonStat, PokemonType,
    };

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
            held_items: Some(vec![PokemonHeldItem {
                item: Some(NamedApiResource {
                    name: Some(String::from("sharp fang")),
                    url: None,
                }),
                version_details: Some(vec![PokemonHeldItemVersion {
                    rarity: Some(20),
                    version: Some(NamedApiResource {
                        name: Some(String::from("x")),
                        url: None,
                    }),
                }]),
            }]),
            location_area_encounters: None,
            moves: None,
            sprites: None,
            species: None,
            stats: Some(vec![PokemonStat {
                effort: Some(0),
                base_stat: Some(15),
                stat: Some(NamedApiResource {
                    name: Some(String::from("speed")),
                    url: None,
                }),
            }]),
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

    #[test]
    fn pokemon_get_renderable_stats() {
        let pokemon = get_stub_pokemon();
        assert_eq!(
            pokemon.get_renderable_stats(),
            vec![Row::new(vec![
                Span::styled("\u{A0}Speed", Style::default().fg(Color::Blue)),
                Span::raw("15"),
            ])]
        )
    }

    #[test]
    fn pokemon_get_renderable_held_items() {
        let pokemon = get_stub_pokemon();
        assert_eq!(
            pokemon.get_renderable_held_items("x-y"),
            vec![Row::new(vec![
                Span::styled("\u{A0}Sharp fang", Style::default().fg(Color::Blue)),
                Span::raw("20%"),
                Span::raw("X"),
            ]),]
        )
    }
}
