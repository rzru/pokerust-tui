use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, utils::PreparePokemonNameForDisplay};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let layout_chunks = prepare_chunks(frame);

    let items: Vec<ListItem> = app
        .pokemon_list
        .items
        .iter()
        .map(|pokemon| {
            let name: &str = pokemon.name.as_ref().unwrap().as_ref();
            let name = name.to_string();

            ListItem::new(name.split_capitalize())
        })
        .collect();

    frame.render_stateful_widget(
        prepare_list(items),
        layout_chunks[0],
        &mut app.pokemon_list.state,
    );

    if let Some(pokemon) = &app.current_pokemon {
        frame.render_widget(layout_block(), layout_chunks[1]);
    } else {
        frame.render_widget(prepare_jumbotron(frame), layout_chunks[1]);
    }
}

fn prepare_chunks(frame: &CrosstermFrame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(frame.size())
}

fn layout_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
}

fn prepare_jumbotron(frame: &CrosstermFrame) -> Paragraph<'static> {
    let screen_height = frame.size().height;
    let number_of_newlines = screen_height / 2 - 7;

    let mut text = vec![];

    for _ in 0..=number_of_newlines {
        text.push(Spans::from("\n"));
    }

    text.extend(vec![
        Spans::from("Hello! Welcome to Pokerust."),
        Spans::from("\n"),
        Spans::from("Use \"Up\" and \"Down\" arrows to navigate through Pokemon list."),
        Spans::from("Press \"Enter\" to see information about needed Pokemon."),
        Spans::from("Press \"Esc\" to leave information screen."),
        Spans::from("Press \"q\" to leave Pokedex."),
    ]);

    Paragraph::new(text)
        .block(layout_block())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn prepare_list(items: Vec<ListItem>) -> List {
    List::new(items)
        .block(
            Block::default()
                .title("Pokemon list")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}
