use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};

use crate::utils::PreparePokemonNameForDisplay;

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
                    format!("\u{A0}{}", name.to_string().split_capitalize()),
                    Style::default().fg(Color::Blue),
                ))
            })
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_as_rows(&self) -> Vec<Row> {
        self.version_details
            .as_ref()
            .and_then(|version_details| {
                Some(
                    version_details
                        .iter()
                        .enumerate()
                        .map(|(idx, item_version)| {
                            Row::new(vec![
                                if idx == 0 {
                                    self.get_renderable_item_name()
                                } else {
                                    Span::raw("")
                                },
                                item_version.get_renderable_rarity(),
                                item_version.get_renderable_version(),
                            ])
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
            .and_then(|rarity| Some(Span::raw(rarity.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_version(&self) -> Span {
        self.version
            .as_ref()
            .and_then(|version| version.name.as_ref())
            .and_then(|name| Some(Span::raw(name.to_string().split_capitalize())))
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
                name: Some(String::from("x-y")),
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
            version_details: Some(vec![
                get_stub_pokemon_held_item_version(),
                get_stub_pokemon_held_item_version(),
            ]),
        }
    }

    #[test]
    fn pokemon_held_item_get_renderable_as_rows() {
        let pokemon_held_item = get_stub_pokemon_held_item();

        assert_eq!(
            pokemon_held_item.get_renderable_as_rows(),
            vec![
                Row::new(vec![
                    Span::styled("Sharp fang", Style::default().fg(Color::Blue)),
                    Span::raw("20"),
                    Span::raw("X Y"),
                ]),
                Row::new(vec![
                    Span::styled("Sharp fang", Style::default().fg(Color::Blue)),
                    Span::raw("20"),
                    Span::raw("X Y"),
                ]),
            ]
        );
    }

    #[test]
    fn pokemon_held_item_version_get_renderable_rarity() {
        let pokemon_held_item_version = get_stub_pokemon_held_item_version();

        assert_eq!(
            pokemon_held_item_version.get_renderable_rarity(),
            Span::raw("20"),
        );
    }

    #[test]
    fn pokemon_held_item_version_get_renderable_version() {
        let pokemon_held_item_version = get_stub_pokemon_held_item_version();

        assert_eq!(
            pokemon_held_item_version.get_renderable_version(),
            Span::raw("X Y"),
        );
    }
}
