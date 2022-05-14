use tui::{
    style::{Color, Style},
    text::Span,
};

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn get_renderable_title(title: &str) -> Span {
    Span::styled(
        format!("\u{A0}{}: ", title),
        Style::default().fg(Color::Blue),
    )
}

pub trait PreparePokemonNameForDisplay {
    fn split_capitalize(self) -> Self;
}

impl PreparePokemonNameForDisplay for String {
    fn split_capitalize(self) -> Self {
        self.split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| uppercase_first_letter(str))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
