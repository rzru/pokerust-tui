use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Row,
};

use super::{pokemon_move::PokemonMoveExt, Pokemon, PokemonAbilityExt, PokemonSpecies};

pub struct ExtendedPokemonInfo {
    pub pokemon: Pokemon,
    pub abilities: Vec<PokemonAbilityExt>,
    pub moves: Vec<PokemonMoveExt>,
    pub species: PokemonSpecies,
}

impl ExtendedPokemonInfo {
    pub fn get_renderable_abilities(&self) -> Vec<Row> {
        self.pokemon
            .abilities
            .as_ref()
            .and_then(|abilities| {
                Some(
                    abilities
                        .iter()
                        .filter_map(|ability| {
                            let extended_ability = self.abilities.iter().find(|extended_ability| {
                                if let Some(item_ability) = ability.ability.as_ref() {
                                    return extended_ability
                                        .name
                                        .as_ref()
                                        .unwrap_or(&"".to_string())
                                        == item_ability.name.as_ref().unwrap_or(&"".to_string());
                                }

                                false
                            });

                            ability.get_renderable_as_row(extended_ability)
                        })
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }

    pub fn get_renderable_basic_info_items(&self) -> Vec<Row> {
        vec![
            Row::new(vec![
                Span::styled(format!("\u{A0}ID"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_id(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Order"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_order(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Name"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_name(),
            ]),
            Row::new(vec![
                Spans::from(Span::styled(
                    format!("\u{A0}Types"),
                    Style::default().fg(Color::Blue),
                )),
                Spans::from(self.pokemon.get_renderable_types()),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Height"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_height(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Weight"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_weight(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Base Experience"),
                    Style::default().fg(Color::Blue),
                ),
                self.pokemon.get_renderable_base_experience(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Base Happiness"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_base_happiness(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Capture Rate"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_capture_rate(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Color"), Style::default().fg(Color::Blue)),
                self.species.get_renderable_color(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Is Legendary"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_is_legendary(),
            ]),
        ]
    }

    pub fn get_renderable_moves(&self) -> Vec<Row> {
        self.pokemon
            .moves
            .as_ref()
            .and_then(|pokemon_moves| {
                Some(
                    pokemon_moves
                        .iter()
                        .filter_map(|pokemon_move| {
                            let extended_pokemon_move = self.moves.iter().find(|extended_move| {
                                if let Some(pokemon_move) = pokemon_move.de_move.as_ref() {
                                    return extended_move.name.as_ref().unwrap_or(&"".to_string())
                                        == pokemon_move.name.as_ref().unwrap_or(&"".to_string());
                                }

                                false
                            });

                            pokemon_move.get_renderable_as_row(extended_pokemon_move)
                        })
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }
}
