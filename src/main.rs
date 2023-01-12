use std::{io, thread, time::Duration};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen}, execute, event::DisableMouseCapture};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders}};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Tetris")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
