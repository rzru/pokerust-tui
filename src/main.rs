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

    app.init().await;

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
                            CurrentMainPageState::BasicInfo => {
                                app.pokemon_moves_list_state.next(app.rendered_moves_count)
                            }
                            CurrentMainPageState::VersionGroupSelection => {
                                app.version_groups.next()
                            }
                        },
                    },
                    KeyCode::Up => match app.selected_part {
                        SelectedPart::List => app.pokemon_list.previous(),
                        SelectedPart::Main => match app.current_main_page_state {
                            CurrentMainPageState::BasicInfo => app
                                .pokemon_moves_list_state
                                .previous(app.rendered_moves_count),
                            CurrentMainPageState::VersionGroupSelection => {
                                app.version_groups.previous()
                            }
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
                            app.on_pokemon_selected(|app| {
                                terminal.draw(|frame| render(frame, app)).unwrap();
                            })
                            .await;
                        }
                        SelectedPart::Main => match app.current_main_page_state {
                            CurrentMainPageState::VersionGroupSelection => {
                                app.on_version_group_selected();
                                app.on_moves_and_abilities_open(|app| {
                                    terminal.draw(|frame| render(frame, app)).unwrap();
                                })
                                .await;
                            }
                            _ => {}
                        },
                    },
                    KeyCode::Char(c) => match app.selected_part {
                        SelectedPart::List => {
                            app.on_search_append(c);
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match app.selected_part {
                        SelectedPart::List => {
                            app.on_search_remove();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
