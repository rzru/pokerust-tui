mod app;
mod http;
mod pokemon_list;
mod stateful_list;
mod ui;
mod utils;

use std::{
    io::{self, Stdout},
    time::Duration,
};

use app::App;
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

    app.fetch_list().await;

    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => app.stateful_list.unselect(),
                    KeyCode::Down => app.stateful_list.next(),
                    KeyCode::Up => app.stateful_list.previous(),
                    _ => {}
                }
            }
        }
    }
}
