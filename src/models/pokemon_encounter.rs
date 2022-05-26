use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tui::{text::Span, widgets::Row};

use crate::utils::PrepareForDisplay;

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonEncounterDetail {
    pub chance: Option<i32>,
    pub max_level: Option<i32>,
    pub min_level: Option<i32>,
    pub method: Option<NamedApiResource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonEncounterVersionDetail {
    pub max_chance: Option<i32>,
    pub version: Option<NamedApiResource>,
    pub encounter_details: Option<Vec<PokemonEncounterDetail>>,
}

impl PokemonEncounterVersionDetail {
    pub fn get_renderable_version(&self) -> Span {
        self.version
            .as_ref()
            .and_then(|version| Some(Span::raw(version.get_name_or_stub().split_capitalize())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_max_chance(&self) -> Span {
        self.max_chance
            .and_then(|max_chance| Some(Span::raw(max_chance.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_methods(&self) -> Span {
        self.encounter_details
            .as_ref()
            .and_then(|encounter_details| {
                let mut items = vec![];
                encounter_details.iter().for_each(|item| {
                    if let Some(method) = item.method.as_ref() {
                        let new_method = method.get_name_or_stub().split_capitalize();
                        if !items.contains(&new_method) {
                            items.push(new_method);
                        }
                    }
                });

                Some(Span::raw(items.join(", ")))
            })
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_levels(&self) -> Span {
        let max_level = self
            .encounter_details
            .as_ref()
            .and_then(|encounter_details| {
                encounter_details
                    .par_iter()
                    .max_by(|a, b| a.max_level.cmp(&b.max_level))
            })
            .and_then(|encounter_detail| encounter_detail.max_level)
            .unwrap_or(0);

        let min_level = self
            .encounter_details
            .as_ref()
            .and_then(|encounter_details| {
                encounter_details
                    .par_iter()
                    .min_by(|a, b| a.min_level.cmp(&b.min_level))
            })
            .and_then(|encounter_detail| encounter_detail.min_level)
            .unwrap_or(0);

        Span::raw(format!("{} - {}", min_level, max_level))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonEncounter {
    pub location_area: Option<NamedApiResource>,
    pub version_details: Option<Vec<PokemonEncounterVersionDetail>>,
}

impl PokemonEncounter {
    pub fn get_renderable_location_area(&self) -> Span {
        self.location_area
            .as_ref()
            .and_then(|location_area| {
                Some(Span::raw(
                    location_area
                        .get_name_or_stub()
                        .to_string()
                        .split_capitalize()
                        .append_padding(),
                ))
            })
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_as_rows(&self, selected_version_group: &str) -> Vec<Row> {
        let version_detail_by_version_groups =
            self.version_details.as_ref().and_then(|version_details| {
                Some(
                    version_details
                        .par_iter()
                        .filter(|version| {
                            if selected_version_group
                                .to_string()
                                .split("-")
                                .map(|item| item.to_string())
                                .collect::<Vec<String>>()
                                .contains(&version.version.as_ref().unwrap().get_name_or_stub())
                            {
                                return true;
                            }

                            false
                        })
                        .collect::<Vec<&PokemonEncounterVersionDetail>>(),
                )
            });

        if let Some(version_detail_by_version_groups) = version_detail_by_version_groups {
            return version_detail_by_version_groups
                .par_iter()
                .map(|version_detail_by_version_group| {
                    Row::new(vec![
                        self.get_renderable_location_area(),
                        version_detail_by_version_group.get_renderable_methods(),
                        version_detail_by_version_group.get_renderable_version(),
                        version_detail_by_version_group.get_renderable_max_chance(),
                        version_detail_by_version_group.get_renderable_levels(),
                    ])
                })
                .collect();
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use tui::{text::Span, widgets::Row};

    use crate::models::NamedApiResource;

    use super::{PokemonEncounter, PokemonEncounterDetail, PokemonEncounterVersionDetail};

    fn get_stubbed_pokemon_encounter_detail() -> PokemonEncounterDetail {
        PokemonEncounterDetail {
            chance: Some(50),
            min_level: Some(1),
            max_level: Some(10),
            method: Some(NamedApiResource {
                name: Some(String::from("walk")),
                url: None,
            }),
        }
    }

    fn get_stubbed_pokemon_encounter_version_detail() -> PokemonEncounterVersionDetail {
        PokemonEncounterVersionDetail {
            max_chance: Some(10),
            version: Some(NamedApiResource {
                name: Some(String::from("y")),
                url: None,
            }),
            encounter_details: Some(vec![
                get_stubbed_pokemon_encounter_detail(),
                PokemonEncounterDetail {
                    chance: Some(50),
                    min_level: Some(20),
                    max_level: Some(40),
                    method: Some(NamedApiResource {
                        name: Some(String::from("headbutt")),
                        url: None,
                    }),
                },
                PokemonEncounterDetail {
                    chance: Some(50),
                    min_level: Some(11),
                    max_level: Some(35),
                    method: Some(NamedApiResource {
                        name: Some(String::from("walk")),
                        url: None,
                    }),
                },
            ]),
        }
    }

    fn get_stubbed_pokemon_encounter() -> PokemonEncounter {
        PokemonEncounter {
            location_area: Some(NamedApiResource {
                name: Some(String::from("kanto-route-3")),
                url: None,
            }),
            version_details: Some(vec![
                get_stubbed_pokemon_encounter_version_detail(),
                PokemonEncounterVersionDetail {
                    max_chance: Some(100),
                    version: Some(NamedApiResource {
                        name: Some(String::from("x")),
                        url: None,
                    }),
                    encounter_details: Some(vec![PokemonEncounterDetail {
                        chance: Some(1),
                        min_level: Some(100),
                        max_level: Some(100),
                        method: Some(NamedApiResource {
                            name: Some(String::from("something")),
                            url: None,
                        }),
                    }]),
                },
            ]),
        }
    }

    #[test]
    fn pokemon_encounter_version_detail_get_renderable_version() {
        let encounter_version_detail = get_stubbed_pokemon_encounter_version_detail();
        assert_eq!(
            encounter_version_detail.get_renderable_version(),
            Span::raw("Y")
        )
    }

    #[test]
    fn pokemon_encounter_version_detail_get_renderable_methods() {
        let encounter_version_detail = get_stubbed_pokemon_encounter_version_detail();
        assert_eq!(
            encounter_version_detail.get_renderable_methods(),
            Span::raw("Walk, Headbutt")
        )
    }

    #[test]
    fn pokemon_encounter_version_detail_get_renderable_max_chance() {
        let encounter_version_detail = get_stubbed_pokemon_encounter_version_detail();
        assert_eq!(
            encounter_version_detail.get_renderable_max_chance(),
            Span::raw("10")
        )
    }

    #[test]
    fn pokemon_encounter_version_detail_get_renderable_levels() {
        let encounter_version_detail = get_stubbed_pokemon_encounter_version_detail();
        assert_eq!(
            encounter_version_detail.get_renderable_levels(),
            Span::raw("1 - 40")
        )
    }

    #[test]
    fn pokemon_encounter_get_renderable_rows() {
        let encounter = get_stubbed_pokemon_encounter();
        assert_eq!(
            encounter.get_renderable_as_rows("x-y"),
            vec![
                Row::new(vec![
                    Span::raw("\u{A0}Kanto Route 3"),
                    Span::raw("Walk, Headbutt"),
                    Span::raw("Y"),
                    Span::raw("10"),
                    Span::raw("1 - 40"),
                ]),
                Row::new(vec![
                    Span::raw("\u{A0}Kanto Route 3"),
                    Span::raw("Something"),
                    Span::raw("X"),
                    Span::raw("100"),
                    Span::raw("100 - 100"),
                ])
            ]
        )
    }
}
