use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    app::{App, SelectedPart},
    models::NamedApiResource,
    utils::PreparePokemonNameForDisplay,
};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let (list_area, search_area, main_area) = prepare_chunks(frame);
    let (list, search) = prepare_list(
        &app.pokemon_list.items_to_render,
        app.search.as_str().to_string(),
        &app.selected_part,
    );

    frame.render_stateful_widget(list, list_area, &mut app.pokemon_list.state);
    frame.render_widget(search, search_area);
    if let SelectedPart::List = app.selected_part {
        frame.set_cursor(
            search_area.x + app.search.width() as u16 + 1,
            search_area.y + 1,
        );
    }

    if let Some(pokemon) = &app.current_pokemon {
        frame.render_widget(layout_block(&app.selected_part), main_area);
    } else {
        frame.render_widget(prepare_jumbotron(&app.selected_part, frame), main_area);
    }
}

fn prepare_chunks(frame: &CrosstermFrame) -> (Rect, Rect, Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(frame.size());

    let list_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(main_chunks[0]);

    return (list_chunks[0], list_chunks[1], main_chunks[1]);
}

fn layout_block(selected_part: &SelectedPart) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(match selected_part {
            SelectedPart::Main => Style::default().fg(Color::Blue),
            SelectedPart::List => Style::default(),
        })
}

fn prepare_jumbotron(selected_part: &SelectedPart, frame: &CrosstermFrame) -> Paragraph<'static> {
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
        .block(layout_block(selected_part))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn prepare_list(
    items: &Vec<NamedApiResource>,
    search: String,
    selected_part: &SelectedPart,
) -> (List<'static>, Paragraph<'static>) {
    let items: Vec<ListItem> = items
        .iter()
        .map(|pokemon| {
            let name: &str = pokemon.name.as_ref().unwrap().as_ref();
            let name = name.to_string();

            ListItem::new(name.split_capitalize())
        })
        .collect();

    let search = Paragraph::new(search).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(match selected_part {
                SelectedPart::List => Style::default().fg(Color::Blue),
                SelectedPart::Main => Style::default(),
            })
            .title("Filter")
            .border_type(BorderType::Rounded),
    );
    let list = List::new(items)
        .block(
            Block::default()
                .title("Pokemon list")
                .borders(Borders::ALL)
                .border_style(match selected_part {
                    SelectedPart::List => Style::default().fg(Color::Blue),
                    SelectedPart::Main => Style::default(),
                })
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    return (list, search);
}
