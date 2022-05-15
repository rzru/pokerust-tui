mod app;
mod http;
mod models;
mod stateful_list;
pub mod switchable_table_state;
mod ui;
mod utils;

use std::{
    io::{self, Stdout},
    time::Duration,
};

use app::{App, CurrentMainPageState, SelectedPart};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use switchable_table_state::SwitchableTableState;
use tokio::join;
use tui::{backend::CrosstermBackend, Terminal};
use ui::render;

const POKEAPI_DEFAULT_URL: &str = "https://pokeapi.co/api/v2/";
const DEFAULT_LIST_QUERY_PARAMS: &str = "?limit=100000&offset=0";

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(app, &mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app(
    mut app: App,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);

    let (pokemon_list, version_groups) =
        join!(app.fetch_pokemon_list(), app.fetch_version_groups());
    app.set_pokemon_list_and_version_groups(pokemon_list, version_groups);
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => app.reset_current_pokemon(),
                    KeyCode::Down => match app.selected_part {
                        SelectedPart::List => app.pokemon_list.next(),
                        SelectedPart::Main => match app.current_main_page_state {
                            CurrentMainPageState::Abilities => app.pokemon_moves_list_state.next(
                                &app.current_pokemon.as_ref().unwrap().get_prepared_moves(
                                    app.selected_version
                                        .as_ref()
                                        .unwrap()
                                        .name
                                        .as_ref()
                                        .unwrap(),
                                ),
                            ),
                            CurrentMainPageState::VersionGroupSelection => {
                                app.version_groups.next()
                            }
                            _ => {}
                        },
                    },
                    KeyCode::Up => match app.selected_part {
                        SelectedPart::List => app.pokemon_list.previous(),
                        SelectedPart::Main => match app.current_main_page_state {
                            CurrentMainPageState::Abilities => {
                                app.pokemon_moves_list_state.previous(
                                    &app.current_pokemon.as_ref().unwrap().get_prepared_moves(
                                        app.selected_version
                                            .as_ref()
                                            .unwrap()
                                            .name
                                            .as_ref()
                                            .unwrap(),
                                    ),
                                )
                            }
                            CurrentMainPageState::VersionGroupSelection => {
                                app.version_groups.previous()
                            }
                            _ => {}
                        },
                    },
                    KeyCode::Left => match app.selected_part {
                        SelectedPart::Main => app.selected_part = SelectedPart::List,
                        _ => {}
                    },
                    KeyCode::Right => match (&app.selected_part, &app.current_pokemon) {
                        (SelectedPart::List, Some(_)) => app.selected_part = SelectedPart::Main,
                        _ => {}
                    },
                    KeyCode::Enter => match app.selected_part {
                        SelectedPart::List => {
                            let pokemon = app.pokemon_list.get_selected().cloned();
                            if let Some(pokemon) = pokemon {
                                app.reset_current_pokemon();
                                app.loading = true;
                                terminal.draw(|frame| render(frame, &mut app))?;
                                app.fetch_pokemon_with_info(&pokemon).await;
                                app.loading = false;
                                app.selected_part = SelectedPart::Main;
                            }
                        }
                        SelectedPart::Main => {
                            match app.current_main_page_state {
                                CurrentMainPageState::VersionGroupSelection => {
                                    app.selected_version = app
                                        .version_groups
                                        .get_selected()
                                        .and_then(|t| Some(t.clone()));
                                }
                                CurrentMainPageState::BasicInfo => {
                                    app.pokemon_moves_list_state = SwitchableTableState::new();
                                    if let Some(current_pokemon) = app.current_pokemon.as_ref() {
                                        if current_pokemon.abilities.is_empty()
                                            && current_pokemon.moves.is_empty()
                                        {
                                            app.loading = true;
                                            terminal.draw(|frame| render(frame, &mut app))?;
                                            app.fetch_abilities_and_moves().await;
                                            app.loading = false;
                                        }
                                    }
                                }
                                _ => {}
                            }
                            app.current_main_page_state = app.current_main_page_state.get_next();
                        }
                    },
                    KeyCode::Char(c) => match app.selected_part {
                        SelectedPart::List => {
                            app.search.push(c);
                            app.filter_list();
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match app.selected_part {
                        SelectedPart::List => {
                            app.search.pop();
                            app.filter_list();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
