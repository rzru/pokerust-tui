use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};

use crate::utils::{get_styled_pokemon_type, PrepareForDisplay};

use super::{FlavorTextEntry, NamedApiResource, VerboseEffect};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

impl PokemonMove {
    pub fn get_renderable_version_group_details(
        &self,
        selected_version: &str,
    ) -> Option<Vec<&PokemonMoveVersion>> {
        self.version_group_details
            .as_ref()
            .and_then(|version_group_details| {
                let items = version_group_details
                    .iter()
                    .filter_map(|version_group_detail| {
                        if let Some(version_group) = version_group_detail.version_group.as_ref() {
                            if version_group.get_name_or_stub() == selected_version {
                                return Some(version_group_detail);
                            }
                        }

                        None
                    })
                    .collect::<Vec<&PokemonMoveVersion>>();

                Some(items)
            })
    }

    pub fn get_renderable_as_row(
        &self,
        extended_move: Option<&PokemonMoveExt>,
        move_version: &PokemonMoveVersion,
    ) -> Option<Row> {
        if let Some(extended_move) = extended_move {
            return Some(Row::new(vec![
                Span::styled(
                    extended_move.get_renderable_name(),
                    Style::default().fg(Color::Blue),
                ),
                Span::raw(extended_move.get_renderable_accuracy()),
                Span::raw(extended_move.get_renderable_pp()),
                Span::raw(extended_move.get_renderable_power()),
                get_styled_pokemon_type(extended_move.get_renderable_type()),
                Span::raw(extended_move.get_renderable_damage_class()),
                Span::raw(move_version.get_renderable_learn_method()),
                Span::raw(move_version.get_renderable_level()),
                Span::raw(extended_move.get_renderable_effect_entry()),
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
            .and_then(|name| Some(name.to_string().split_capitalize().append_padding()))
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

impl PokemonMoveVersion {
    pub fn get_renderable_learn_method(&self) -> String {
        self.move_learn_method
            .as_ref()
            .and_then(|method| method.name.as_ref())
            .and_then(|name| Some(name.to_string().split_capitalize()))
            .unwrap_or(String::from("-"))
    }

    pub fn get_renderable_level(&self) -> String {
        self.level_learned_at
            .as_ref()
            .and_then(|level| Some(level.to_string()))
            .unwrap_or(String::from("-"))
    }
}

#[cfg(test)]
mod tests {
    use tui::{
        style::{Color, Style},
        text::Span,
        widgets::Row,
    };

    use crate::models::NamedApiResource;

    use super::{PokemonMove, PokemonMoveExt, PokemonMoveVersion, VerboseEffect};

    fn get_stub_pokemon_move_version() -> PokemonMoveVersion {
        PokemonMoveVersion {
            level_learned_at: Some(25),
            move_learn_method: Some(NamedApiResource {
                name: Some(String::from("level up")),
                url: None,
            }),
            version_group: Some(NamedApiResource {
                name: Some(String::from("x-y")),
                url: None,
            }),
        }
    }

    fn get_stub_pokemon_move() -> PokemonMove {
        PokemonMove {
            de_move: Some(NamedApiResource {
                name: Some(String::from("pound")),
                url: None,
            }),
            version_group_details: Some(vec![
                get_stub_pokemon_move_version(),
                PokemonMoveVersion {
                    level_learned_at: Some(0),
                    move_learn_method: Some(NamedApiResource {
                        name: Some(String::from("egg")),
                        url: None,
                    }),
                    version_group: Some(NamedApiResource {
                        name: Some(String::from("sword-shield")),
                        url: None,
                    }),
                },
            ]),
        }
    }

    fn get_stub_pokemon_move_ext() -> PokemonMoveExt {
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
                name: Some(String::from("special")),
                url: None,
            }),
        }
    }

    #[test]
    fn pokemon_move_get_renderable_version_group_details() {
        let pokemon_move = get_stub_pokemon_move();
        let pokemon_move_version_details = pokemon_move
            .get_renderable_version_group_details("x-y")
            .unwrap();

        assert_eq!(pokemon_move_version_details.len(), 1);
        assert_eq!(
            pokemon_move_version_details
                .first()
                .unwrap()
                .level_learned_at
                .unwrap(),
            pokemon_move
                .version_group_details
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .level_learned_at
                .unwrap()
        )
    }

    #[test]
    fn pokemon_move_get_renderable_as_row() {
        let pokemon_move = get_stub_pokemon_move();
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        let pokemon_move_version = get_stub_pokemon_move_version();

        assert_eq!(
            pokemon_move.get_renderable_as_row(Some(&pokemon_move_ext), &pokemon_move_version),
            Some(Row::new(vec![
                Span::styled("\u{A0}Pound", Style::default().fg(Color::Blue)),
                Span::raw("100"),
                Span::raw("20"),
                Span::raw("60"),
                Span::styled("Normal ", Style::default().fg(Color::Rgb(170, 170, 153))),
                Span::raw("Special"),
                Span::raw("Level up"),
                Span::raw("25"),
                Span::raw("short effect"),
            ])),
        );
        assert_eq!(
            pokemon_move.get_renderable_as_row(None, &pokemon_move_version),
            None
        );
    }

    #[test]
    fn pokemon_move_ext_get_renderable_name() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(
            pokemon_move_ext.get_renderable_name(),
            String::from("\u{A0}Pound")
        )
    }

    #[test]
    fn pokemon_move_ext_get_renderable_pp() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(pokemon_move_ext.get_renderable_pp(), String::from("20"))
    }

    #[test]
    fn pokemon_move_ext_get_renderable_accuracy() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(
            pokemon_move_ext.get_renderable_accuracy(),
            String::from("100")
        )
    }

    #[test]
    fn pokemon_move_ext_get_renderable_power() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(pokemon_move_ext.get_renderable_power(), String::from("60"))
    }

    #[test]
    fn pokemon_move_ext_get_renderable_type() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(
            pokemon_move_ext.get_renderable_type(),
            String::from("normal")
        )
    }

    #[test]
    fn pokemon_move_ext_get_renderable_damage_class() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(
            pokemon_move_ext.get_renderable_damage_class(),
            String::from("Special")
        )
    }

    #[test]
    fn pokemon_move_ext_get_renderable_effect_entry() {
        let pokemon_move_ext = get_stub_pokemon_move_ext();
        assert_eq!(
            pokemon_move_ext.get_renderable_effect_entry(),
            String::from("short effect")
        )
    }

    #[test]
    fn pokemon_move_version_get_renderable_level() {
        let pokemon_move_version = get_stub_pokemon_move_version();
        assert_eq!(
            pokemon_move_version.get_renderable_level(),
            String::from("25")
        )
    }

    #[test]
    fn pokemon_move_version_get_renderable_learn_method() {
        let pokemon_move_version = get_stub_pokemon_move_version();
        assert_eq!(
            pokemon_move_version.get_renderable_learn_method(),
            String::from("Level up")
        )
    }
}
