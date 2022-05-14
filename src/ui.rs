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
    app::{App, ExtendedPokemonInfo, SelectedPart},
    utils::PreparePokemonNameForDisplay,
};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let (list_area, search_area, main_area) = prepare_chunks(frame);

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

    let list_style = match app.selected_part {
        SelectedPart::List => Style::default().fg(Color::Blue),
        SelectedPart::Main => Style::default(),
    };

    let main_style = match app.selected_part {
        SelectedPart::Main => Style::default().fg(Color::Blue),
        SelectedPart::List => Style::default(),
    };

    let search = Paragraph::new(app.search.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(list_style)
            .title("Filter")
            .border_type(BorderType::Rounded),
    );

    let list = List::new(pokemon_items_to_render)
        .block(
            Block::default()
                .title("Pokemon list")
                .borders(Borders::ALL)
                .border_style(list_style)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let main_block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(main_style);

    if let Some(current_pokemon) = &app.current_pokemon {
        let ExtendedPokemonInfo {
            pokemon, species, ..
        } = current_pokemon;

        let main_block_chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(16),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .margin(1)
            .split(main_area);

        let main_information_text = vec![
            Spans::from(vec![
                get_renderable_title("ID"),
                pokemon.get_renderable_id(),
            ]),
            Spans::from(vec![
                get_renderable_title("Order"),
                pokemon.get_renderable_order(),
            ]),
            Spans::from(vec![
                get_renderable_title("Name"),
                pokemon.get_renderable_name(),
            ]),
            Spans::from({
                let mut spans = vec![get_renderable_title("Types")];
                spans.extend(pokemon.get_renderable_types());

                spans
            }),
            Spans::from(vec![
                get_renderable_title("Height"),
                pokemon.get_renderable_height(),
            ]),
            Spans::from(vec![
                get_renderable_title("Weight"),
                pokemon.get_renderable_weight(),
            ]),
            Spans::from(vec![
                get_renderable_title("Base Experience"),
                pokemon.get_renderable_base_experience(),
            ]),
            Spans::from(vec![
                get_renderable_title("Base Happiness"),
                species.get_renderable_base_happiness(),
            ]),
            Spans::from(vec![
                get_renderable_title("Capture Rate"),
                species.get_renderable_capture_rate(),
            ]),
            Spans::from(vec![
                get_renderable_title("Color"),
                species.get_renderable_color(),
            ]),
            Spans::from(vec![
                get_renderable_title("Is Legendary"),
                species.get_renderable_is_legendary(),
            ]),
        ];

        let pokemon_stats_table = Table::new(pokemon.get_renderable_stats())
            .header(
                Row::new(vec!["\u{A0}Name", "Base Stat"]).style(Style::default().fg(Color::Blue)),
            )
            .block(Block::default().title(Spans::from(Span::styled(
                "\u{A0}Base Stats",
                Style::default().add_modifier(Modifier::BOLD),
            ))))
            .widths(&[Constraint::Percentage(15), Constraint::Percentage(30)])
            .column_spacing(1);

        let pokedex_numbers_table = Table::new(species.get_renderable_pokedex_numbers())
            .header(
                Row::new(vec!["\u{A0}Order Number", "Region"])
                    .style(Style::default().fg(Color::Blue)),
            )
            .block(Block::default().title(Spans::from(Span::styled(
                "\u{A0}Pokedex Numbers",
                Style::default().add_modifier(Modifier::BOLD),
            ))))
            .widths(&[Constraint::Percentage(15), Constraint::Percentage(30)])
            .column_spacing(1);

        let main_information_paragraph = Paragraph::new(main_information_text)
            .block(Block::default().title(Spans::from(Span::styled(
                "\u{A0}Main information",
                Style::default().add_modifier(Modifier::BOLD),
            ))))
            .wrap(Wrap { trim: true });

        frame.render_widget(main_block, main_area);
        frame.render_widget(main_information_paragraph, main_block_chunks[0]);
        frame.render_widget(pokemon_stats_table, main_block_chunks[1]);
        frame.render_widget(pokedex_numbers_table, main_block_chunks[2]);
    } else {
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

        let paragraph = Paragraph::new(text)
            .block(main_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, main_area);
    }

    frame.render_stateful_widget(list, list_area, &mut app.pokemon_list.state);
    frame.render_widget(search, search_area);
    if let SelectedPart::List = app.selected_part {
        frame.set_cursor(
            search_area.x + app.search.width() as u16 + 1,
            search_area.y + 1,
        );
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

pub fn get_renderable_title(title: &str) -> Span {
    Span::styled(
        format!("\u{A0}{}: ", title),
        Style::default().fg(Color::Blue),
    )
}
