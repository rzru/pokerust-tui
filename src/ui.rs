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
use rayon::prelude::*;

use crate::{
    app::{App, CurrentMainPageState, SelectedPart},
    models::ExtendedPokemonInfo,
    utils::PrepareForDisplay,
};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let (list_area, search_area, main_area) = prepare_chunks(frame);
    let version_group_selection_area = prepare_version_group_selection_area(main_area);
    let (basic_info_area, right_area) = prepare_main_block_chunks(main_area);
    let (list_style, main_style) = get_styles(app);

    render_list(frame, app, list_area, list_style);
    render_search(frame, app, search_area, list_style);
    render_main_block(frame, app, main_area, main_style);

    if app.loading {
        return;
    }

    if let Some(current_pokemon) = app.current_pokemon.as_mut() {
        match app.current_main_page_state {
            CurrentMainPageState::BasicInfo => {
                let selected_version_group = app.selected_version_group
                    .as_ref()
                    .unwrap()
                    .name
                    .as_ref()
                    .unwrap();

                let basic_info_table = get_renderable_basic_info_table(current_pokemon);
                let pokemon_stats_table = get_renderable_pokemon_stats_table(current_pokemon);
                let (pokemon_held_items_table, held_items_count) = get_renderable_pokemon_held_items_table(
                    current_pokemon,
                    selected_version_group
                );
                let (pokemon_encounters_table, encounters_count) = get_renderable_pokemon_encounters_table(
                    current_pokemon,
                    selected_version_group
                );
                let pokedex_numbers_table = get_renderable_pokedex_numbers_table(current_pokemon);
                let (abilities_table, abilities_count) = get_renderable_pokemon_abilities_table(current_pokemon);
                let (moves_table, moves_count) = get_renderable_pokemon_moves_table(
                    current_pokemon,
                    selected_version_group
                );
                app.rendered_moves_count = Some(moves_count);

                let (abilities_area, encounters_area, moves_area) =
                    prepare_main_block_right_chunks(right_area, encounters_count as u16, abilities_count as u16);
                let (basic_info_area, pokemon_stats_area, held_items_area, pokedex_numbers_area) =
                    prepare_basic_info_chunks(basic_info_area, held_items_count as u16);

                frame.render_widget(basic_info_table, basic_info_area);
                frame.render_widget(pokemon_stats_table, pokemon_stats_area);
                frame.render_widget(pokedex_numbers_table, pokedex_numbers_area);
                frame.render_widget(pokemon_held_items_table, held_items_area);
                frame.render_widget(pokemon_encounters_table, encounters_area);
                frame.render_widget(abilities_table, abilities_area);
                frame.render_stateful_widget(
                    moves_table,
                    moves_area,
                    &mut app.pokemon_moves_list_state.0,
                );
            }
            CurrentMainPageState::VersionGroupSelection => {
                render_version_groups_selection_list(frame, app, version_group_selection_area)
            }
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
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .margin(1)
        .direction(Direction::Horizontal)
        .split(area);

    (main_block_chunks[0], main_block_chunks[1])
}

fn prepare_main_block_right_chunks(area: Rect, encounters_count: u16, abilities_count: u16) -> (Rect, Rect, Rect) {
    let main_block_chunks = Layout::default()
        .constraints([Constraint::Length(abilities_count + 3), Constraint::Length(encounters_count + 3), Constraint::Percentage(90)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    (main_block_chunks[0], main_block_chunks[1], main_block_chunks[2])
}

fn prepare_version_group_selection_area(area: Rect) -> Rect {
    let main_block_chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .direction(Direction::Vertical)
        .split(area);

    main_block_chunks[0]
}

fn prepare_basic_info_chunks(area: Rect, held_items_count: u16) -> (Rect, Rect, Rect, Rect) {
    let left_block_chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(13),
                Constraint::Length(8),
                Constraint::Length(held_items_count + 2),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(area);

    (
        left_block_chunks[0],
        left_block_chunks[1],
        left_block_chunks[2],
        left_block_chunks[3],
    )
}

fn render_version_groups_selection_list(frame: &mut CrosstermFrame, app: &mut App, area: Rect) {
    let version_groups_to_render: Vec<ListItem> = app
        .version_groups
        .items_to_render
        .par_iter()
        .map(|version_group| {
            let name: &str = version_group.name.as_ref().unwrap().as_ref();
            let name = name.to_string().split_capitalize();

            ListItem::new(name)
        })
        .collect();

    let list = List::new(version_groups_to_render)
        .block(
            Block::default().title(Span::styled(
                String::from("Select generation")
                    .append_padding()
                    .append_padding(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.version_groups.state);
}

fn render_list(frame: &mut CrosstermFrame, app: &mut App, area: Rect, style: Style) {
    let pokemon_items_to_render: Vec<ListItem> = app
        .pokemon_list
        .items_to_render
        .par_iter()
        .map(|pokemon| {
            let name: &str = pokemon.name.as_ref().unwrap().as_ref();
            let name = name.to_string().split_capitalize();

            ListItem::new(name)
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

    if app.current_pokemon.is_none() || app.loading {
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
        .widths(&[Constraint::Percentage(60), Constraint::Percentage(40)])
        .column_spacing(1)
}

fn get_renderable_basic_info_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.get_renderable_basic_info_items())
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Basic Info",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[Constraint::Percentage(60), Constraint::Percentage(40)])
        .column_spacing(1)
}

fn get_renderable_pokemon_held_items_table<'a>(
    current_pokemon: &'a ExtendedPokemonInfo,
    selected_version_group: &str,
) -> (Table<'a>, usize) {
    let held_items = 
        current_pokemon
            .pokemon
            .get_renderable_held_items(selected_version_group);
    let held_items_count = held_items.len();
    let table = Table::new(held_items)
    .block(Block::default().title(Spans::from(Span::styled(
        "\u{A0}Held items",
        Style::default().add_modifier(Modifier::BOLD),
    ))))
    .widths(&[
        Constraint::Length(14),
        Constraint::Length(6),
        Constraint::Percentage(100),
    ])
    .column_spacing(1);
    
    (table, held_items_count)
}

fn get_renderable_pokemon_encounters_table<'a>(
    current_pokemon: &'a ExtendedPokemonInfo,
    selected_version_group: &str,
) -> (Table<'a>, usize) {
    let encounters = 
        current_pokemon
            .get_renderable_encounters(selected_version_group);
    let encounters_count = encounters.len();
    let table = Table::new(encounters)
    .block(Block::default().title(Spans::from(Span::styled(
        "\u{A0}Encounters",
        Style::default().add_modifier(Modifier::BOLD),
    ))))
    .header(
        Row::new(vec!["\u{A0}Location", "Methods", "Version", "Max Chance", "Levels"])
            .style(Style::default().fg(Color::Blue)),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ])
    .column_spacing(1);

    (table, encounters_count)
}

fn get_renderable_pokemon_abilities_table(current_pokemon: &ExtendedPokemonInfo) -> (Table, usize) {
    let abilities = current_pokemon.get_renderable_abilities();
    let abilities_count = abilities.len();
    let table = Table::new(abilities)
        .header(
            Row::new(vec!["\u{A0}Name", "Effect", "Is Hidden"])
                .style(Style::default().fg(Color::Blue)),
        )
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Abilities",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(75),
            Constraint::Percentage(15),
        ])
        .column_spacing(1);

    (table, abilities_count)
}

fn get_renderable_pokemon_moves_table<'a>(
    current_pokemon: &'a ExtendedPokemonInfo,
    selected_version_group: &str,
) -> (Table<'a>, usize) {
    let moves = current_pokemon.get_renderable_moves(selected_version_group);
    let moves_len = &moves.len();
    let table = Table::new(moves)
        .header(
            Row::new(vec![
                "\u{A0}Name",
                "Acc.",
                "PP",
                "Pow.",
                "Type",
                "Class",
                "Method",
                "Lv.",
                "Effect",
            ])
            .style(Style::default().fg(Color::Blue)),
        )
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Moves",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[
            Constraint::Length(16),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(4),
            Constraint::Percentage(100),
        ])
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .column_spacing(1);

    (table, *moves_len)
}

fn get_renderable_pokedex_numbers_table(current_pokemon: &ExtendedPokemonInfo) -> Table {
    Table::new(current_pokemon.species.get_renderable_pokedex_numbers())
        .block(Block::default().title(Spans::from(Span::styled(
            "\u{A0}Pokedex Numbers",
            Style::default().add_modifier(Modifier::BOLD),
        ))))
        .widths(&[Constraint::Percentage(60), Constraint::Percentage(40)])
        .column_spacing(1)
}

fn get_main_block_text<'a>(frame: &CrosstermFrame, app: &App) -> Vec<Spans<'a>> {
    let mut text = vec![];

    let welcoming_text = vec![
        Spans::from("Hello! Welcome to Pokerust."),
        Spans::from("\n"),
        Spans::from("Use \"Up\" and \"Down\" to navigate through Pokemon/Moves list."),
        Spans::from("Press \"Enter\" to see information about needed Pokemon."),
        Spans::from("Use \"Left\" and \"Right\" switch between Pokemon list and Pokemon Info."),
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
