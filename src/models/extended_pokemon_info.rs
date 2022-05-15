use std::cmp::Ordering;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Row,
};

use crate::utils::PrepareForDisplay;

use super::{
    pokemon_move::PokemonMoveExt, Pokemon, PokemonAbilityExt, PokemonMove, PokemonMoveVersion,
    PokemonSpecies,
};

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
        let paint_blue = |string: String| Span::styled(string, Style::default().fg(Color::Blue));

        vec![
            Row::new(vec![
                paint_blue(String::from("ID").append_padding()),
                self.pokemon.get_renderable_id(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Order").append_padding()),
                self.pokemon.get_renderable_order(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Name").append_padding()),
                self.pokemon.get_renderable_name(),
            ]),
            Row::new(vec![
                Spans::from(paint_blue(String::from("Types").append_padding())),
                Spans::from(self.pokemon.get_renderable_types()),
            ]),
            Row::new(vec![
                paint_blue(String::from("Height").append_padding()),
                self.pokemon.get_renderable_height(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Weight").append_padding()),
                self.pokemon.get_renderable_weight(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Base Experience").append_padding()),
                self.pokemon.get_renderable_base_experience(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Base Happiness").append_padding()),
                self.species.get_renderable_base_happiness(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Capture Rate").append_padding()),
                self.species.get_renderable_capture_rate(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Color").append_padding()),
                self.species.get_renderable_color(),
            ]),
            Row::new(vec![
                paint_blue(String::from("Is Legendary").append_padding()),
                self.species.get_renderable_is_legendary(),
            ]),
        ]
    }

    pub fn get_prepared_moves(
        &self,
        selected_version: &str,
    ) -> Vec<(&PokemonMove, Vec<&PokemonMoveVersion>)> {
        self.pokemon
            .moves
            .as_ref()
            .and_then(|pokemon_moves| {
                Some(
                    pokemon_moves
                        .iter()
                        .filter_map(|pokemon_move| {
                            pokemon_move
                                .get_renderable_version_group_details(selected_version)
                                .and_then(|version_group_details| {
                                    if version_group_details.is_empty() {
                                        return None;
                                    }

                                    Some((pokemon_move, version_group_details))
                                })
                        })
                        .collect(),
                )
            })
            .unwrap_or(vec![])
    }

    pub fn get_renderable_moves(&self, selected_version: &str) -> Vec<Row> {
        let mut prepared_moves = self.get_prepared_moves(selected_version);
        prepared_moves.sort_by(|(.., first_move_versions), (.., second_move_versions)| {
            if let (Some(first_move_version), Some(second_move_version)) =
                (first_move_versions.first(), second_move_versions.first())
            {
                let level_cmp = first_move_version
                    .level_learned_at
                    .as_ref()
                    .and_then(|first_level_learned_at| {
                        second_move_version.level_learned_at.as_ref().and_then(
                            |second_level_learned_at| {
                                Some(first_level_learned_at.cmp(second_level_learned_at))
                            },
                        )
                    })
                    .unwrap_or(Ordering::Equal);

                let move_learn_method_cmp = first_move_version
                    .move_learn_method
                    .as_ref()
                    .and_then(|first_move_learn_method| {
                        second_move_version.move_learn_method.as_ref().and_then(
                            |second_move_learn_method| {
                                Some(
                                    first_move_learn_method
                                        .get_name_or_stub()
                                        .cmp(&second_move_learn_method.get_name_or_stub()),
                                )
                            },
                        )
                    })
                    .unwrap_or(Ordering::Equal);

                return level_cmp.then(move_learn_method_cmp);
            }

            Ordering::Equal
        });

        prepared_moves
            .iter()
            .filter_map(|(pokemon_move, move_versions)| {
                let extended_pokemon_move = self.moves.iter().find(|extended_move| {
                    if let Some(pokemon_move) = pokemon_move.de_move.as_ref() {
                        return extended_move.name.as_ref().unwrap_or(&"".to_string())
                            == pokemon_move.name.as_ref().unwrap_or(&"".to_string());
                    }

                    false
                });

                pokemon_move
                    .get_renderable_as_row(extended_pokemon_move, move_versions.first().unwrap())
            })
            .collect()
    }
}
