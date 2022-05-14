use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};

use crate::utils::PreparePokemonNameForDisplay;

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: Option<NamedApiResource>,
    pub effort: Option<i32>,
    pub base_stat: Option<i32>,
}

impl PokemonStat {
    pub fn get_renderable_as_row(&self) -> Row {
        let stat_name = self
            .stat
            .as_ref()
            .and_then(|stat| stat.name.as_ref())
            .and_then(|stat_name| {
                Some(format!(
                    "\u{A0}{}: ",
                    stat_name.to_string().split_capitalize()
                ))
            })
            .unwrap_or(String::new());

        let base_value = self
            .base_stat
            .and_then(|base_stat| Some(base_stat.to_string()))
            .unwrap_or(String::new());

        Row::new(vec![
            Span::styled(stat_name, Style::default().fg(Color::Blue)),
            Span::raw(base_value),
        ])
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

    use super::PokemonStat;

    fn get_stub_pokemon_stat() -> PokemonStat {
        PokemonStat {
            stat: Some(NamedApiResource {
                name: Some(String::from("speed")),
                url: None,
            }),
            effort: Some(0),
            base_stat: Some(15),
        }
    }

    #[test]
    fn pokemon_stat_get_renderable_as_row() {
        let pokemon_stat = get_stub_pokemon_stat();
        assert_eq!(
            pokemon_stat.get_renderable_as_row(),
            Row::new(vec![
                Span::styled("\u{A0}Speed: ", Style::default().fg(Color::Blue)),
                Span::raw("15"),
            ])
        )
    }
}
