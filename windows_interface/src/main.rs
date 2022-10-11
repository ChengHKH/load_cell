pub mod app;
pub mod ui;

use crate::app::App;
use std::{
    io,
    time::{Duration, Instant},
    thread,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    widgets::{Widget, Block, Borders},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, DisableMouseCapture, Event, KeyCode},
    execute,
};

fn main() -> Result<(), io::Error> {
    // setup tui
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // initiate app and run
    let app = App::new("Load Cell Reader");
    let run_app = run(&mut terminal, app, Duration::from_millis(200));

    // exit tui
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen,DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_secs(5));
    return Ok(());
}