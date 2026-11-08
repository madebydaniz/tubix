use std::io::{self, IsTerminal};

use crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use ratatui::DefaultTerminal;

pub struct TerminalSession {
    terminal: DefaultTerminal,
}

impl TerminalSession {
    pub fn enter() -> io::Result<Self> {
        if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
            return Err(io::Error::other("an interactive terminal is required"));
        }

        match ratatui::try_init() {
            Ok(terminal) => {
                let session = Self { terminal };
                execute!(io::stdout(), EnableMouseCapture)?;
                Ok(session)
            }
            Err(error) => {
                restore();
                Err(error)
            }
        }
    }

    pub fn terminal_mut(&mut self) -> &mut DefaultTerminal {
        &mut self.terminal
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        restore();
    }
}

fn restore() {
    let _ = execute!(io::stdout(), DisableMouseCapture);
    let _ = ratatui::try_restore();
    let _ = execute!(io::stdout(), Show);
}
