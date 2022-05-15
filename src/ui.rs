use std::{io::Stdout, vec};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    app::App,
    models::{CurrentMainPageState, ExtendedPokemonInfo, SelectedPart},
    utils::PreparePokemonNameForDisplay,
};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let (list_area, search_area, main_area) = prepare_chunks(frame);
    let (basic_info_area, held_items_area) = prepare_main_block_chunks(main_area);
    let (abilities_area, moves_area) = prepare_main_block_second_page_chunks(main_area);
    let (basic_info_area, pokemon_stats_area, pokedex_numbers_area) =
        prepare_basic_info_chunks(basic_info_area);
    let (list_style, main_style) = get_styles(app);

    render_list(frame, app, list_area, list_style);
    render_search(frame, app, search_area, list_style);
    render_main_block(frame, app, main_area, main_style);

    if let Some(current_pokemon) = app.current_pokemon.as_ref() {
        if let CurrentMainPageState::BasicInfo = app.current_main_page_state {
            let basic_info_table = get_renderable_basic_info_table(current_pokemon);
            let pokemon_stats_table = get_renderable_pokemon_stats_table(current_pokemon);
            let pokemon_held_items_table = get_renderable_pokemon_held_items_table(current_pokemon);
            let pokedex_numbers_table = get_renderable_pokedex_numbers_table(current_pokemon);

            frame.render_widget(basic_info_table, basic_info_area);
            frame.render_widget(pokemon_stats_table, pokemon_stats_area);
            frame.render_widget(pokedex_numbers_table, pokedex_numbers_area);
            frame.render_widget(pokemon_held_items_table, held_items_area);
        }

        if let CurrentMainPageState::Abilities = app.current_main_page_state {
            let abilities_table = get_renderable_pokemon_abilities_table(current_pokemon);
            frame.render_widget(abilities_table, abilities_area);
        }
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

fn prepare_main_block_chunks(area: Rect) -> (Rect, Rect) {
    let main_block_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(1)
        .direction(Direction::Horizontal)
        .split(area);

    (main_block_chunks[0], main_block_chunks[1])
}

fn prepare_main_block_second_page_chunks(area: Rect) -> (Rect, Rect) {
    let main_block_chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(60)].as_ref())
        .margin(1)
        .direction(Direction::Vertical)
        .split(area);

    (main_block_chunks[0], main_block_chunks[1])
}

fn prepare_basic_info_chunks(area: Rect) -> (Rect, Rect, Rect) {
    let left_block_chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(15),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(area);

    (
        left_block_chunks[0],
        left_block_chunks[1],
        left_block_chunks[2],
    )
}

fn render_list(frame: &mut CrosstermFrame, app: &mut App, area: Rect, style: Style) {
    let pokemon_items_to_render: Vec<ListItem> = app
        .pokemon_list
        .items_to_render
        .iter()
        .map(|pokemon| {
            let name: &str = pokemon.name.as_ref().unwrap().as_ref();
            let name = name.to_string();

            ListItem::new(name.split_capitalize())
        })
        .collect();

    let list = List::new(pokemon_items_to_render)
        .block(
            Block::default()
                .title("Pokemon list")
                .borders(Borders::ALL)
                .border_style(style)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.pokemon_list.state);
}

fn render_search(frame: &mut CrosstermFrame, app: &App, area: Rect, style: Style) {
    let search = Paragraph::new(app.search.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .title("Filter")
            .border_type(BorderType::Rounded),
    );

    if let SelectedPart::List = app.selected_part {
        frame.set_cursor(area.x + app.search.width() as u16 + 1, area.y + 1);
    }

    frame.render_widget(search, area);
}

fn render_main_block(frame: &mut CrosstermFrame, app: &App, area: Rect, style: Style) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(style);

    if app.current_pokemon.is_none() {
        let paragraph = Paragraph::new(get_main_block_text(frame, app))
            .block(main_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    } else {
        frame.render_widget(main_block, area);
    }
}

fn get_styles(app: &App) -> (Style, Style) {
    let highlighted = Style::default().fg(Color::Blue);
    let default = Style::default();
    match app.selected_part {
        SelectedPart::List => (highlighted, default),
        SelectedPart::Main => (default, highlighted),
    }
}

fn get_renderable_pokemon_stats_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.pokemon.get_renderable_stats())
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Base Stats",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)])
        .column_spacing(1)
}

fn get_renderable_basic_info_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.get_renderable_basic_info_items())
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Basic Info",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)])
        .column_spacing(1)
}

fn get_renderable_pokemon_held_items_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.pokemon.get_renderable_held_items())
        .header(
            Row::new(vec!["\u{A0}Name", "Rarity", "Game Version"])
                .style(Style::default().fg(Color::Blue)),
        )
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Held items",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .column_spacing(1)
}

fn get_renderable_pokemon_abilities_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.get_renderable_abilities())
        .header(
            Row::new(vec!["\u{A0}Is Hidden", "Name", "Effect"])
                .style(Style::default().fg(Color::Blue)),
        )
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Abilities",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(75),
        ])
        .column_spacing(1)
}

fn get_renderable_pokedex_numbers_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.species.get_renderable_pokedex_numbers())
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Pokedex Numbers",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)])
        .column_spacing(1)
}

fn get_main_block_text<'a>(frame: &CrosstermFrame, app: &App) -> Vec<Spans<'a>> {
    let mut text = vec![];

    let welcoming_text = vec![
        Spans::from("Hello! Welcome to Pokerust."),
        Spans::from("\n"),
        Spans::from("Use \"Up\" and \"Down\" arrows to navigate through Pokemon list."),
        Spans::from("Press \"Enter\" to see information about needed Pokemon."),
        Spans::from("Press \"Esc\" to leave information screen."),
        Spans::from("Press \"q\" to leave Pokedex."),
    ];
    let loading_text = vec![Spans::from(Span::styled(
        "Loading...",
        Style::default().add_modifier(Modifier::BOLD),
    ))];

    let current_text = if app.loading {
        loading_text
    } else {
        welcoming_text
    };

    let screen_height = frame.size().height;
    let number_of_newlines = screen_height / 2 - current_text.len() as u16 / 2 - 3;

    for _ in 0..=number_of_newlines {
        text.push(Spans::from("\n"));
    }

    text.extend(current_text);

    text
}
