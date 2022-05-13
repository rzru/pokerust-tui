use serde::{Deserialize, Serialize};
use tui::{text::Span, widgets::Row};

use crate::utils::PreparePokemonNameForDisplay;

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
    pub fn get_renderable_base_happiness(&self) -> Span {
        Span::raw(self.base_happiness.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_capture_rate(&self) -> Span {
        Span::raw(self.capture_rate.as_ref().unwrap().to_string())
    }

    pub fn get_renderable_color(&self) -> Span {
        Span::raw(
            self.color
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                .to_string()
                .split_capitalize(),
        )
    }

    pub fn get_renderable_pokedex_numbers(&self) -> Vec<Row> {
        self.pokedex_numbers
            .as_ref()
            .unwrap()
            .iter()
            .map(|pokedex_number| {
                Row::new(vec![
                    format!("\u{A0}{}", pokedex_number.get_renderable_entry_number()),
                    pokedex_number.get_renderable_pokedex_name(),
                ])
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokedexNumber {
    entry_number: Option<i32>,
    pokedex: Option<NamedApiResource>,
}

impl PokedexNumber {
    fn get_renderable_entry_number(&self) -> String {
        self.entry_number.unwrap().to_string()
    }

    fn get_renderable_pokedex_name(&self) -> String {
        self.pokedex
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .to_string()
            .split_capitalize()
    }
}
