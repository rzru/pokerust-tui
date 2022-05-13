mod app;
mod http;
mod models;
mod stateful_list;
mod ui;
mod utils;

use std::{
    io::{self, Stdout},
    time::Duration,
};

use app::{App, SelectedPart};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};
use ui::render;

const POKEAPI_DEFAULT_URL: &str = "https://pokeapi.co/api/v2/";

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

    app.fetch_pokemon_list().await;

    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => {
                        app.reset_current_pokemon();
                        app.selected_part = SelectedPart::List;
                    }
                    KeyCode::Down => {
                        if let SelectedPart::List = app.selected_part {
                            app.pokemon_list.next()
                        }
                    }
                    KeyCode::Up => {
                        if let SelectedPart::List = app.selected_part {
                            app.pokemon_list.previous()
                        }
                    }
                    KeyCode::Left => {
                        if let SelectedPart::Main = app.selected_part {
                            app.selected_part = SelectedPart::List
                        }
                    }
                    KeyCode::Right => {
                        if let (SelectedPart::List, Some(_)) =
                            (&app.selected_part, &app.current_pokemon)
                        {
                            app.selected_part = SelectedPart::Main
                        }
                    }
                    KeyCode::Enter => {
                        let pokemon = app.pokemon_list.get_selected().cloned();

                        if let Some(pokemon) = pokemon {
                            app.reset_current_pokemon();
                            app.loading = true;
                            terminal.draw(|frame| render(frame, &mut app))?;
                            app.fetch_pokemon_with_info(&pokemon).await;
                            app.loading = false;
                        }
                    }
                    KeyCode::Char(c) => {
                        if let SelectedPart::List = app.selected_part {
                            app.search.push(c);
                            app.filter_list();
                        }
                    }
                    KeyCode::Backspace => {
                        if let SelectedPart::List = app.selected_part {
                            app.search.pop();
                            app.filter_list();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
