use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};
use rayon::prelude::*;

use crate::utils::PrepareForDisplay;

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
    pub pokedex_numbers: Option<Vec<PokedexNumber>>,
}

impl PokemonSpecies {
    pub fn get_renderable_is_legendary(&self) -> Span {
        self.is_legendary
            .and_then(|is_legendary| Some(Span::raw(if is_legendary { "Yes" } else { "No" })))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_base_happiness(&self) -> Span {
        self.base_happiness
            .and_then(|base_happiness| Some(Span::raw(base_happiness.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_capture_rate(&self) -> Span {
        self.capture_rate
            .and_then(|capture_rate| Some(Span::raw(capture_rate.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_color(&self) -> Span {
        self.color
            .as_ref()
            .and_then(|color| color.name.as_ref())
            .and_then(|color| Some(Span::raw(color.to_string().split_capitalize())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_pokedex_numbers(&self) -> Vec<Row> {
        self.pokedex_numbers
            .as_ref()
            .and_then(|pokedex_numbers| {
                Some(
                    pokedex_numbers
                        .par_iter()
                        .map(|pokedex_number| {
                            Row::new(vec![
                                Span::styled(
                                    pokedex_number
                                        .get_renderable_pokedex_name()
                                        .append_padding(),
                                    Style::default().fg(Color::Blue),
                                ),
                                Span::raw(pokedex_number.get_renderable_entry_number()),
                            ])
                        })
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokedexNumber {
    entry_number: Option<i32>,
    pokedex: Option<NamedApiResource>,
}

impl PokedexNumber {
    fn get_renderable_entry_number(&self) -> String {
        self.entry_number
            .and_then(|entry_number| Some(entry_number.to_string()))
            .unwrap_or(String::new())
    }

    fn get_renderable_pokedex_name(&self) -> String {
        self.pokedex
            .as_ref()
            .and_then(|pokedex| pokedex.name.as_ref())
            .and_then(|pokedex| Some(pokedex.to_string().split_capitalize()))
            .unwrap_or(String::new())
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

    use super::{PokedexNumber, PokemonSpecies};

    fn get_stub_pokedex_number() -> PokedexNumber {
        PokedexNumber {
            entry_number: Some(1),
            pokedex: Some(NamedApiResource {
                name: Some(String::from("kanto")),
                url: None,
            }),
        }
    }

    fn get_stub_species() -> PokemonSpecies {
        return PokemonSpecies {
            gender_rate: Some(4),
            capture_rate: Some(100),
            color: Some(NamedApiResource {
                name: Some(String::from("brown")),
                url: None,
            }),
            base_happiness: Some(50),
            is_legendary: Some(false),
            evolution_chain: None,
            flavor_text_entries: None,
            pokedex_numbers: Some(vec![get_stub_pokedex_number()]),
        };
    }

    #[test]
    fn pokemon_species_get_renderable_is_legendary() {
        let species = get_stub_species();
        assert_eq!(species.get_renderable_is_legendary(), Span::raw("No"));
    }

    #[test]
    fn pokemon_species_get_renderable_base_happiness() {
        let species = get_stub_species();
        assert_eq!(species.get_renderable_base_happiness(), Span::raw("50"));
    }

    #[test]
    fn pokemon_species_get_renderable_capture_rate() {
        let species = get_stub_species();
        assert_eq!(species.get_renderable_capture_rate(), Span::raw("100"));
    }

    #[test]
    fn pokemon_species_get_renderable_color() {
        let species = get_stub_species();
        assert_eq!(species.get_renderable_color(), Span::raw("Brown"));
    }

    #[test]
    fn pokemon_species_get_renderable_pokedex_numbers() {
        let species = get_stub_species();
        assert_eq!(
            species.get_renderable_pokedex_numbers(),
            vec![Row::new(vec![
                Span::styled("\u{A0}Kanto", Style::default().fg(Color::Blue)),
                Span::raw("1")
            ])]
        );
    }

    #[test]
    fn pokedex_number_get_renderable_entry_number() {
        let pokedex_number = get_stub_pokedex_number();
        assert_eq!(
            pokedex_number.get_renderable_entry_number(),
            String::from("1")
        );
    }

    #[test]
    fn pokedex_number_get_renderable_pokedex_name() {
        let pokedex_number = get_stub_pokedex_number();
        assert_eq!(
            pokedex_number.get_renderable_pokedex_name(),
            String::from("Kanto")
        );
    }
}
