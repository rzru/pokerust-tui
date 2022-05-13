use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
};

use crate::utils::PreparePokemonNameForDisplay;

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    pub slot: Option<i32>,
    #[serde(rename = "type")]
    pub de_type: Option<NamedApiResource>,
}

impl PokemonType {
    pub fn get_renderable(&self) -> Span {
        let empty = Span::raw("");

        if let Some(de_type) = &self.de_type {
            let name = de_type.name.as_ref().unwrap();

            return match name.as_str() {
                "normal" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(170, 170, 153)),
                ),
                "fire" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(255, 68, 34)),
                ),
                "water" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(51, 153, 255)),
                ),
                "electric" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(255, 204, 51)),
                ),
                "grass" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(119, 204, 85)),
                ),
                "ice" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(102, 204, 255)),
                ),
                "fighting" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(197, 85, 68)),
                ),
                "poison" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(170, 85, 153)),
                ),
                "ground" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(221, 187, 85)),
                ),
                "flying" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(136, 153, 255)),
                ),
                "psychic" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(255, 85, 153)),
                ),
                "bug" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(170, 187, 34)),
                ),
                "rock" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(187, 170, 32)),
                ),
                "ghost" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(102, 102, 187)),
                ),
                "dragon" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(119, 102, 238)),
                ),
                "dark" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(119, 85, 68)),
                ),
                "steel" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(170, 170, 187)),
                ),
                "fairy" => Span::styled(
                    format!("{} ", name).split_capitalize(),
                    Style::default().fg(Color::Rgb(238, 153, 238)),
                ),
                _ => empty,
            };
        }

        empty
    }
}
