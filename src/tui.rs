use std::io::{self, IsTerminal, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub running: bool,
    restored: bool,
}

pub fn init() -> Result<App> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(App {
        terminal,
        running: true,
        restored: false,
    })
}

pub fn is_interactive() -> bool {
    io::stdout().is_terminal()
}

pub fn run_with_tui<F>(task: F) -> Result<()>
where
    F: FnOnce() -> Result<()> + Send + 'static,
{
    let mut app = init()?;
    let handle = std::thread::spawn(task);

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.running = false;
                }
            }
        }

        app.draw()?;

        if !app.running || handle.is_finished() {
            break;
        }
    }

    app.draw()?;
    app.restore()?;

    handle
        .join()
        .map_err(|_| anyhow::anyhow!("background thread panicked"))?
}

impl App {
    pub fn restore(&mut self) -> Result<()> {
        if self.restored {
            return Ok(());
        }
        self.restored = true;
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();
            let block = Block::default()
                .title(" metaphaze ")
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL);
            frame.render_widget(block, area);
        })?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
