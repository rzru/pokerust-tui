use serde::{Deserialize, Serialize};
use tui::text::Span;

use crate::utils::get_styled_pokemon_type;

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    pub slot: Option<i32>,
    #[serde(rename = "type")]
    pub de_type: Option<NamedApiResource>,
}

impl PokemonType {
    pub fn get_renderable(&self) -> Span {
        self.de_type
            .as_ref()
            .and_then(|test| test.name.as_ref())
            .and_then(|name| Some(get_styled_pokemon_type(name.to_string())))
            .unwrap_or(Span::raw(""))
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::Span,
    };

    use crate::models::NamedApiResource;

    use super::PokemonType;

    fn get_stub_type(type_name: &str) -> PokemonType {
        PokemonType {
            slot: None,
            de_type: Some(NamedApiResource {
                name: Some(String::from(type_name)),
                url: None,
            }),
        }
    }

    #[test]
    fn pokemon_type_get_renderable() {
        let pokemon_type = get_stub_type("fairy");
        assert_eq!(
            pokemon_type.get_renderable(),
            Span::styled("Fairy ", Style::default().fg(Color::Rgb(238, 153, 238)))
        )
    }

    #[test]
    fn pokemon_type_get_renderable_wrong_type() {
        let pokemon_type = get_stub_type("");
        assert_eq!(pokemon_type.get_renderable(), Span::raw(""))
    }
}
