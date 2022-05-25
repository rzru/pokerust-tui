use serde::{Serialize, Deserialize};
use tui::{widgets::Row, text::Span};

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
    pub encounter_details: Option<Vec<PokemonEncounterDetail>>
}

impl PokemonEncounterVersionDetail {
    pub fn get_renderable_version(&self) -> Span {
        self
            .version
            .as_ref()
            .and_then(|version| Some(Span::raw(version.get_name_or_stub().split_capitalize())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_max_chance(&self) -> Span {
        self
            .max_chance
            .and_then(|max_chance| Some(Span::raw(max_chance.to_string())))
            .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_methods(&self) -> Span {
        self
            .encounter_details
            .as_ref()
            .and_then(|encounter_details| {
                let mut items = vec![];
                encounter_details
                    .iter()
                    .for_each(|item| {
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
                encounter_details.iter().max_by(|a, b| a.max_level.cmp(&b.max_level))
            })
            .and_then(|encounter_detail| encounter_detail.max_level)
            .unwrap_or(0);

        let min_level = self
            .encounter_details
            .as_ref()
            .and_then(|encounter_details| {
                encounter_details.iter().min_by(|a, b| a.min_level.cmp(&b.min_level))
            })
            .and_then(|encounter_detail| encounter_detail.min_level)
            .unwrap_or(0);

        Span::raw(format!("{} - {}", min_level, max_level))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonEncounter {
    pub location_area: Option<NamedApiResource>,
    pub version_details: Option<Vec<PokemonEncounterVersionDetail>>
}

impl PokemonEncounter {
    pub fn get_renderable_location_area(&self) -> Span {
        self
        .location_area
        .as_ref()
        .and_then(|location_area| Some(Span::raw(location_area.get_name_or_stub().to_string().split_capitalize().append_padding())))
        .unwrap_or(Span::raw(""))
    }

    pub fn get_renderable_as_rows(&self, selected_version_group: &str) -> Vec<Row> {
        let mut result = vec![];

        let version_detail_by_version_group = self
            .version_details
            .as_ref()
            .and_then(|version_details| {
                version_details
                    .iter()
                    .find(|version| {
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
            });

        if let Some(version_detail_by_version_group) = version_detail_by_version_group {
            result.push(Row::new(vec![
                self.get_renderable_location_area(),
                version_detail_by_version_group.get_renderable_methods(),
                version_detail_by_version_group.get_renderable_version(),
                version_detail_by_version_group.get_renderable_max_chance(),
                version_detail_by_version_group.get_renderable_levels(),
            ]));
        }

        result
    }
}