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
            .and_then(|verbose_effects| {
                verbose_effects
                    .first()
                    .and_then(|verbose_effect| Some(verbose_effect.get_effect()))
            })
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

#[cfg(test)]
mod tests {
    use tui::{text::Span, widgets::Row};

    use crate::models::NamedApiResource;

    use super::{PokemonAbility, PokemonAbilityExt, VerboseEffect};

    fn get_stub_verbose_effect() -> VerboseEffect {
        VerboseEffect {
            effect: Some(String::from("effect")),
            short_effect: Some(String::from("short effect")),
            language: Some(NamedApiResource {
                name: Some(String::from("en")),
                url: None,
            }),
        }
    }

    fn get_stub_pokemon_ability(is_hidden: bool) -> PokemonAbility {
        PokemonAbility {
            is_hidden: Some(is_hidden),
            slot: None,
            ability: Some(NamedApiResource {
                name: Some(String::from("test")),
                url: None,
            }),
        }
    }

    fn get_stub_pokemon_ability_ext() -> PokemonAbilityExt {
        PokemonAbilityExt {
            id: Some(1),
            name: Some(String::from("test")),
            effect_entries: Some(vec![get_stub_verbose_effect()]),
            flavor_text_entries: None,
        }
    }

    #[test]
    fn pokemon_ability_get_renderable_is_hidden_work_with_hidden() {
        assert_eq!(
            Span::raw("Yes"),
            get_stub_pokemon_ability(true).get_renderable_is_hidden()
        )
    }

    #[test]
    fn pokemon_ability_get_renderable_is_hidden_work_with_not_hidden() {
        assert_eq!(
            Span::raw("No"),
            get_stub_pokemon_ability(false).get_renderable_is_hidden()
        )
    }

    #[test]
    fn pokemon_ability_get_renderable_as_row() {
        let extended_pokemon_info = get_stub_pokemon_ability_ext();
        assert_eq!(
            Some(Row::new(vec![
                Span::raw("No"),
                Span::raw("Test"),
                Span::raw("effect")
            ])),
            get_stub_pokemon_ability(false).get_renderable_as_row(Some(&extended_pokemon_info))
        )
    }

    #[test]
    fn pokemon_ability_ext_get_renderable_name() {
        assert_eq!(
            String::from("Test"),
            get_stub_pokemon_ability_ext().get_renderable_name()
        )
    }

    #[test]
    fn pokemon_ability_ext_get_renderable_effect_entry() {
        assert_eq!(
            String::from("effect"),
            get_stub_pokemon_ability_ext().get_renderable_effect_entry()
        )
    }

    #[test]
    fn verbose_effect_get_effect() {
        assert_eq!(
            String::from("effect"),
            get_stub_verbose_effect().get_effect()
        )
    }

    #[test]
    fn verbose_effect_get_language() {
        assert_eq!(String::from("en"), get_stub_verbose_effect().get_language())
    }
}
