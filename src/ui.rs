use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::{app::App, utils::uppercase_first_letter, APP_LABEL};

type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub fn render(frame: &mut CrosstermFrame, app: &mut App) {
    let layout_chunks = prepare_chunks(frame);

    let items: Vec<ListItem> = app
        .stateful_list
        .items
        .iter()
        .map(|pokemon| {
            let name = pokemon.name.as_ref().unwrap().as_ref();

            ListItem::new(uppercase_first_letter(name))
        })
        .collect();

    frame.render_stateful_widget(
        prepare_list(items),
        layout_chunks[0],
        &mut app.stateful_list.state,
    );
    frame.render_widget(prepare_main_block(), layout_chunks[1]);
}

fn prepare_chunks(frame: &mut CrosstermFrame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(frame.size())
}

fn prepare_main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(APP_LABEL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
}

fn prepare_list(items: Vec<ListItem>) -> List {
    List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}
