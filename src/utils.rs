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

pub fn get_styled_pokemon_type(name: String) -> Span<'static> {
    match name.as_str() {
        "normal" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(170, 170, 153)),
        ),
        "fire" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(255, 68, 34)),
        ),
        "water" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(51, 153, 255)),
        ),
        "electric" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(255, 204, 51)),
        ),
        "grass" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(119, 204, 85)),
        ),
        "ice" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(102, 204, 255)),
        ),
        "fighting" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(197, 85, 68)),
        ),
        "poison" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(170, 85, 153)),
        ),
        "ground" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(221, 187, 85)),
        ),
        "flying" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(136, 153, 255)),
        ),
        "psychic" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(255, 85, 153)),
        ),
        "bug" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(170, 187, 34)),
        ),
        "rock" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(187, 170, 32)),
        ),
        "ghost" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(102, 102, 187)),
        ),
        "dragon" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(119, 102, 238)),
        ),
        "dark" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(119, 85, 68)),
        ),
        "steel" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(170, 170, 187)),
        ),
        "fairy" => Span::styled(
            format!("{} ", name).split_capitalize(),
            Style::default().fg(Color::Rgb(238, 153, 238)),
        ),
        _ => Span::raw(""),
    }
}

pub trait PrepareForDisplay {
    fn split_capitalize(self) -> Self;
    fn append_padding(self) -> Self;
}

impl PrepareForDisplay for String {
    fn split_capitalize(self) -> Self {
        self.split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| uppercase_first_letter(str))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn append_padding(self) -> Self {
        format!("\u{A0}{}", self)
    }
}
