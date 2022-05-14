use serde::{Deserialize, Serialize};
use tui::{text::Span, widgets::Row};

use crate::utils::PreparePokemonNameForDisplay;

use super::{FlavorTextEntry, NamedApiResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: Option<bool>,
    pub slot: Option<i32>,
    pub ability: Option<NamedApiResource>,
}

impl PokemonAbility {
    pub fn get_renderable_is_hidden(&self) -> Span {
        Span::raw(if self.is_hidden.unwrap_or(false) {
            "Yes"
        } else {
            "No"
        })
    }

    pub fn get_renderable_as_row(
        &self,
        extended_ability: Option<&PokemonAbilityExt>,
    ) -> Option<Row> {
        if let Some(ability) = extended_ability {
            return Some(Row::new(vec![
                self.get_renderable_is_hidden(),
                Span::raw(ability.get_renderable_name()),
                Span::raw(ability.get_renderable_effect_entry()),
            ]));
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbilityExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub effect_entries: Option<Vec<VerboseEffect>>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}

impl PokemonAbilityExt {
    pub fn effects(&self) -> (String, String) {
        let mut effect = String::new();
        let mut short_effect = String::new();

        let effects: Vec<&VerboseEffect> = self
            .effect_entries
            .as_ref()
            .unwrap()
            .iter()
            .filter(|ee| {
                let mut ok = false;

                if let Some(language) = &ee.language {
                    ok = language.name.as_ref().unwrap_or(&String::new()) == "en"
                }

                ok
            })
            .collect();

        if let Some(verbose_effects) = effects.first() {
            if let Some(fetched_effect) = verbose_effects.effect.as_ref() {
                effect = fetched_effect.to_string()
            }

            if let Some(fetched_short_effect) = verbose_effects.short_effect.as_ref() {
                short_effect = fetched_short_effect.to_string()
            }
        }

        (effect, short_effect)
    }

    pub fn get_renderable_name(&self) -> String {
        self.name
            .as_ref()
            .and_then(|name| Some(name.to_string().split_capitalize()))
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
            .and_then(|verbose_effects| Some(verbose_effects.first().unwrap().get_effect()))
            .unwrap_or(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerboseEffect {
    pub effect: Option<String>,
    pub short_effect: Option<String>,
    pub language: Option<NamedApiResource>,
}

impl VerboseEffect {
    pub fn get_language(&self) -> String {
        self.language
            .as_ref()
            .and_then(|language| language.name.as_ref())
            .and_then(|language| Some(language.to_string()))
            .unwrap_or(String::new())
    }

    pub fn get_effect(&self) -> String {
        self.effect.as_ref().unwrap_or(&"".to_string()).to_string()
    }
}
