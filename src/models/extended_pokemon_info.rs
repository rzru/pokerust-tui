use std::cmp::Ordering;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Row,
};
use rayon::prelude::*;

use crate::utils::PrepareForDisplay;

use super::{pokemon_move::PokemonMoveExt, Pokemon, PokemonAbilityExt, PokemonSpecies, PokemonEncounter};

pub struct ExtendedPokemonInfo {
    pub pokemon: Pokemon,
    pub abilities: Vec<PokemonAbilityExt>,
    pub moves: Vec<PokemonMoveExt>,
    pub species: PokemonSpecies,
    pub encounters: Vec<PokemonEncounter>,
}

impl ExtendedPokemonInfo {
    pub fn get_renderable_abilities(&self) -> Vec<Row> {
        self.pokemon
            .abilities
            .as_ref()
            .and_then(|abilities| {
                Some(
                    abilities
                        .par_iter()
                        .filter_map(|ability| {
                            let extended_ability = self.abilities.par_iter().find_any(|extended_ability| {
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

    pub fn get_renderable_moves(&self, selected_version_group: &str) -> Vec<Row> {
        let mut prepared_moves = self
            .pokemon
            .moves
            .as_ref()
            .and_then(|pokemon_moves| {
                Some(
                    pokemon_moves
                        .par_iter()
                        .filter_map(|pokemon_move| {
                            pokemon_move
                                .get_renderable_version_group_details(selected_version_group)
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
            .unwrap_or(vec![]);

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
            .par_iter()
            .filter_map(|(pokemon_move, move_versions)| {
                let extended_pokemon_move = self.moves.par_iter().find_any(|extended_move| {
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

    pub fn get_renderable_encounters(&self, selected_version_group: &str) -> Vec<Row> {
        self
            .encounters
            .par_iter()
            .flat_map(|encounter| encounter.get_renderable_as_rows(selected_version_group))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::{Span, Spans},
        widgets::Row,
    };

    use crate::models::{
        NamedApiResource, Pokemon, PokemonAbility, PokemonAbilityExt, PokemonHeldItem,
        PokemonHeldItemVersion, PokemonMove, PokemonMoveExt, PokemonMoveVersion, PokemonSpecies,
        PokemonStat, PokemonType, VerboseEffect,
    };

    use super::ExtendedPokemonInfo;

    fn get_stub_extended_pokemon_info() -> ExtendedPokemonInfo {
        ExtendedPokemonInfo {
            pokemon: Pokemon {
                id: Some(1),
                name: Some(String::from("raichu")),
                base_experience: Some(200),
                height: Some(10),
                is_default: None,
                order: Some(1),
                weight: Some(100),
                abilities: Some(vec![PokemonAbility {
                    is_hidden: Some(true),
                    slot: None,
                    ability: Some(NamedApiResource {
                        name: Some(String::from("guts")),
                        url: None,
                    }),
                }]),
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
                            name: Some(String::from("x-y")),
                            url: None,
                        }),
                    }]),
                }]),
                location_area_encounters: None,
                moves: Some(vec![
                    PokemonMove {
                        de_move: Some(NamedApiResource {
                            name: Some(String::from("swift")),
                            url: None,
                        }),
                        version_group_details: Some(vec![PokemonMoveVersion {
                            level_learned_at: Some(25),
                            move_learn_method: Some(NamedApiResource {
                                name: Some(String::from("level up")),
                                url: None,
                            }),
                            version_group: Some(NamedApiResource {
                                name: Some(String::from("x-y")),
                                url: None,
                            }),
                        }]),
                    },
                    PokemonMove {
                        de_move: Some(NamedApiResource {
                            name: Some(String::from("pound")),
                            url: None,
                        }),
                        version_group_details: Some(vec![PokemonMoveVersion {
                            level_learned_at: Some(0),
                            move_learn_method: Some(NamedApiResource {
                                name: Some(String::from("egg")),
                                url: None,
                            }),
                            version_group: Some(NamedApiResource {
                                name: Some(String::from("x-y")),
                                url: None,
                            }),
                        }]),
                    },
                ]),
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
            },
            abilities: vec![PokemonAbilityExt {
                id: Some(1),
                name: Some(String::from("guts")),
                effect_entries: Some(vec![VerboseEffect {
                    effect: None,
                    short_effect: Some(String::from("short effect")),
                    language: Some(NamedApiResource {
                        name: Some(String::from("en")),
                        url: None,
                    }),
                }]),
                flavor_text_entries: None,
            }],
            moves: vec![
                PokemonMoveExt {
                    id: Some(1),
                    name: Some(String::from("swift")),
                    accuracy: Some(100),
                    pp: Some(20),
                    power: Some(60),
                    pk_type: Some(NamedApiResource {
                        name: Some(String::from("normal")),
                        url: None,
                    }),
                    flavor_text_entries: None,
                    effect_entries: Some(vec![VerboseEffect {
                        effect: None,
                        short_effect: Some(String::from("short effect")),
                        language: Some(NamedApiResource {
                            name: Some(String::from("en")),
                            url: None,
                        }),
                    }]),
                    damage_class: Some(NamedApiResource {
                        name: Some(String::from("special")),
                        url: None,
                    }),
                },
                PokemonMoveExt {
                    id: Some(1),
                    name: Some(String::from("pound")),
                    accuracy: Some(100),
                    pp: Some(20),
                    power: Some(60),
                    pk_type: Some(NamedApiResource {
                        name: Some(String::from("normal")),
                        url: None,
                    }),
                    flavor_text_entries: None,
                    effect_entries: Some(vec![VerboseEffect {
                        effect: None,
                        short_effect: Some(String::from("short effect")),
                        language: Some(NamedApiResource {
                            name: Some(String::from("en")),
                            url: None,
                        }),
                    }]),
                    damage_class: Some(NamedApiResource {
                        name: Some(String::from("physical")),
                        url: None,
                    }),
                },
            ],
            species: PokemonSpecies {
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
                pokedex_numbers: None,
            },
            encounters: vec![]
        }
    }

    #[test]
    fn extended_pokemon_info_get_renderable_abilities() {
        let extended_pokemon_info = get_stub_extended_pokemon_info();
        assert_eq!(
            extended_pokemon_info.get_renderable_abilities(),
            vec![Row::new(vec![
                Span::raw("\u{A0}Guts"),
                Span::raw("short effect"),
                Span::raw("Yes"),
            ])]
        )
    }

    #[test]
    fn extended_pokemon_info_get_renderable_basic_info_items() {
        let extended_pokemon_info = get_stub_extended_pokemon_info();
        assert_eq!(
            extended_pokemon_info.get_renderable_basic_info_items(),
            vec![
                Row::new(vec![
                    Span::styled("\u{A0}ID", Style::default().fg(Color::Blue)),
                    Span::raw("1"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Order", Style::default().fg(Color::Blue)),
                    Span::raw("1"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Name", Style::default().fg(Color::Blue)),
                    Span::raw("Raichu"),
                ]),
                Row::new(vec![
                    Spans::from(Span::styled(
                        "\u{A0}Types",
                        Style::default().fg(Color::Blue)
                    )),
                    Spans::from(vec![Span::styled(
                        "Electric ",
                        Style::default().fg(Color::Rgb(255, 204, 51))
                    )]),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Height", Style::default().fg(Color::Blue)),
                    Span::raw("10"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Weight", Style::default().fg(Color::Blue)),
                    Span::raw("100"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Base Experience", Style::default().fg(Color::Blue)),
                    Span::raw("200"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Base Happiness", Style::default().fg(Color::Blue)),
                    Span::raw("50"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Capture Rate", Style::default().fg(Color::Blue)),
                    Span::raw("100"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Color", Style::default().fg(Color::Blue)),
                    Span::raw("Brown"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Is Legendary", Style::default().fg(Color::Blue)),
                    Span::raw("No"),
                ]),
            ]
        )
    }

    #[test]
    fn extended_pokemon_info_get_renderable_moves() {
        let extended_pokemon_info = get_stub_extended_pokemon_info();
        assert_eq!(
            extended_pokemon_info.get_renderable_moves("x-y"),
            vec![
                Row::new(vec![
                    Span::styled("\u{A0}Pound", Style::default().fg(Color::Blue),),
                    Span::raw("100"),
                    Span::raw("20"),
                    Span::raw("60"),
                    Span::styled("Normal ", Style::default().fg(Color::Rgb(170, 170, 153))),
                    Span::raw("Physical"),
                    Span::raw("Egg"),
                    Span::raw("0"),
                    Span::raw("short effect"),
                ]),
                Row::new(vec![
                    Span::styled("\u{A0}Swift", Style::default().fg(Color::Blue),),
                    Span::raw("100"),
                    Span::raw("20"),
                    Span::raw("60"),
                    Span::styled("Normal ", Style::default().fg(Color::Rgb(170, 170, 153))),
                    Span::raw("Special"),
                    Span::raw("Level up"),
                    Span::raw("25"),
                    Span::raw("short effect"),
                ])
            ]
        )
    }
}
