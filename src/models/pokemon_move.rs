use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Text,
    widgets::Row,
};

use crate::utils::{get_styled_pokemon_type, PreparePokemonNameForDisplay};

use super::{FlavorTextEntry, NamedApiResource, VerboseEffect};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

impl PokemonMove {
    pub fn get_renderable_as_row(&self, extended_move: Option<&PokemonMoveExt>) -> Option<Row> {
        if let Some(extended_move) = extended_move {
            return Some(Row::new(vec![
                Text::styled(
                    format!("\u{A0}{}", extended_move.get_renderable_name()),
                    Style::default().fg(Color::Blue),
                ),
                Text::raw(extended_move.get_renderable_accuracy()),
                Text::raw(extended_move.get_renderable_pp()),
                Text::raw(extended_move.get_renderable_power()),
                Text::from(get_styled_pokemon_type(extended_move.get_renderable_type())),
                Text::raw(extended_move.get_renderable_damage_class()),
                Text::raw(extended_move.get_renderable_effect_entry()),
            ]));
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub accuracy: Option<i32>,
    pub pp: Option<i32>,
    pub power: Option<i32>,
    #[serde(rename = "type")]
    pub pk_type: Option<NamedApiResource>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
    pub damage_class: Option<NamedApiResource>,
    pub effect_entries: Option<Vec<VerboseEffect>>,
}

impl PokemonMoveExt {
    pub fn get_renderable_name(&self) -> String {
        self.name
            .as_ref()
            .and_then(|name| Some(name.to_string().split_capitalize()))
            .unwrap_or(String::new())
    }

    pub fn get_renderable_accuracy(&self) -> String {
        self.accuracy
            .as_ref()
            .and_then(|accuracy| Some(accuracy.to_string()))
            .unwrap_or(String::from("-"))
    }

    pub fn get_renderable_pp(&self) -> String {
        self.pp
            .as_ref()
            .and_then(|pp| Some(pp.to_string()))
            .unwrap_or(String::from("-"))
    }

    pub fn get_renderable_power(&self) -> String {
        self.power
            .as_ref()
            .and_then(|power| Some(power.to_string()))
            .unwrap_or(String::from("-"))
    }

    pub fn get_renderable_type(&self) -> String {
        self.pk_type
            .as_ref()
            .and_then(|pokemon_type| pokemon_type.name.as_ref())
            .and_then(|pokemon_type_name| Some(pokemon_type_name.to_string()))
            .unwrap_or(String::new())
    }

    pub fn get_renderable_damage_class(&self) -> String {
        self.damage_class
            .as_ref()
            .and_then(|damage_class| damage_class.name.as_ref())
            .and_then(|damage_class_name| Some(damage_class_name.to_string().split_capitalize()))
            .unwrap_or(String::new())
    }

    pub fn get_renderable_effect_entry(&self) -> String {
        self.effect_entries
            .as_ref()
            .and_then(|verbose_effects| {
                Some(
                    verbose_effects
                        .iter()
                        .filter(|verbose_effect| verbose_effect.get_language() == "en")
                        .collect::<Vec<&VerboseEffect>>(),
                )
            })
            .and_then(|verbose_effects| {
                verbose_effects
                    .first()
                    .and_then(|verbose_effect| Some(verbose_effect.get_effect()))
            })
            .unwrap_or(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveVersion {
    pub move_learn_method: Option<NamedApiResource>,
    pub version_group: Option<NamedApiResource>,
    pub level_learned_at: Option<i32>,
}
