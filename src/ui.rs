use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
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

    let items: Vec<ListItem> = app
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
    let list = List::new(items)
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
            pokemon,
            abilities,
            moves,
            species,
        } = current_pokemon;

        let text = vec![
            Spans::from(vec![
                Span::styled("\u{A0}ID: ", Style::default().fg(Color::Blue)),
                Span::raw(pokemon.id.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Order: ", Style::default().fg(Color::Blue)),
                Span::raw(pokemon.order.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Name: ", Style::default().fg(Color::Blue)),
                Span::raw(
                    pokemon
                        .name
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .split_capitalize(),
                ),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Height: ", Style::default().fg(Color::Blue)),
                Span::raw(pokemon.height.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Weight: ", Style::default().fg(Color::Blue)),
                Span::raw(pokemon.weight.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Base Experience: ", Style::default().fg(Color::Blue)),
                Span::raw(pokemon.base_experience.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Base Happiness: ", Style::default().fg(Color::Blue)),
                Span::raw(species.base_happiness.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Capture Rate: ", Style::default().fg(Color::Blue)),
                Span::raw(species.capture_rate.as_ref().unwrap().to_string()),
            ]),
            Spans::from(vec![
                Span::styled("\u{A0}Color: ", Style::default().fg(Color::Blue)),
                Span::raw(
                    species
                        .color
                        .as_ref()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .split_capitalize(),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(main_block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, main_area);
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
