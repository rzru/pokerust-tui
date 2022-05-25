use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};
use rayon::prelude::*;

use crate::utils::PrepareForDisplay;

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub item: Option<NamedApiResource>,
    pub version_details: Option<Vec<PokemonHeldItemVersion>>,
}

impl PokemonHeldItem {
    pub fn get_renderable_item_name(&self) -> Span {
        self.item
            .as_ref()
            .and_then(|item| item.name.as_ref())
            .and_then(|name| {
                Some(Span::styled(
                    name.to_string().split_capitalize().append_padding(),
                    Style::default().fg(Color::Blue),
                ))
            })
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_as_rows(&self, selected_version_group: &str) -> Vec<Row> {
        self.version_details
            .as_ref()
            .and_then(|version_details| {
                Some(
                    version_details
                        .par_iter()
                        .filter_map(|item_version| {
                            item_version.version.as_ref().and_then(|version| {
                                if selected_version_group
                                    .to_string()
                                    .split("-")
                                    .map(|item| item.to_string())
                                    .collect::<Vec<String>>()
                                    .contains(&version.get_name_or_stub())
                                {
                                    return Some(Row::new(vec![
                                        self.get_renderable_item_name(),
                                        item_version.get_renderable_rarity(),
                                        item_version.get_renderable_version(),
                                    ]));
                                }

                                None
                            })
                        })
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItemVersion {
    pub rarity: Option<i32>,
    pub version: Option<NamedApiResource>,
}

impl PokemonHeldItemVersion {
    pub fn get_renderable_rarity(&self) -> Span {
        self.rarity
            .as_ref()
            .and_then(|rarity| Some(Span::raw(format!("{}%", rarity.to_string()))))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_version(&self) -> Span {
        self.version
            .as_ref()
            .and_then(|version| version.name.as_ref())
            .and_then(|version| Some(Span::raw(version.to_string().split_capitalize())))
            .unwrap_or(Span::raw(""))
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::Span,
        widgets::Row,
    };

    use crate::models::NamedApiResource;

    use super::{PokemonHeldItem, PokemonHeldItemVersion};

    fn get_stub_pokemon_held_item_version() -> PokemonHeldItemVersion {
        PokemonHeldItemVersion {
            rarity: Some(20),
            version: Some(NamedApiResource {
                name: Some(String::from("x")),
                url: None,
            }),
        }
    }

    fn get_stub_pokemon_held_item() -> PokemonHeldItem {
        PokemonHeldItem {
            item: Some(NamedApiResource {
                name: Some(String::from("sharp fang")),
                url: None,
            }),
            version_details: Some(vec![get_stub_pokemon_held_item_version()]),
        }
    }

    #[test]
    fn pokemon_held_item_get_renderable_as_rows() {
        let pokemon_held_item = get_stub_pokemon_held_item();

        assert_eq!(
            pokemon_held_item.get_renderable_as_rows("x-y"),
            vec![Row::new(vec![
                Span::styled("\u{A0}Sharp fang", Style::default().fg(Color::Blue)),
                Span::raw("20%"),
                Span::raw("X"),
            ]),]
        );
    }

    #[test]
    fn pokemon_held_item_version_get_renderable_rarity() {
        let pokemon_held_item_version = get_stub_pokemon_held_item_version();

        assert_eq!(
            pokemon_held_item_version.get_renderable_rarity(),
            Span::raw("20%"),
        );
    }

    #[test]
    fn pokemon_held_item_version_get_renderable_version() {
        let pokemon_held_item_version = get_stub_pokemon_held_item_version();

        assert_eq!(
            pokemon_held_item_version.get_renderable_version(),
            Span::raw("X"),
        );
    }
}
