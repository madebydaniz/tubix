pub mod cli;
pub mod event;
pub mod runtime;

use std::io;

use ratatui::DefaultTerminal;
use tokio::sync::{mpsc, watch};

use crate::system::signals;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Quit,
    ToggleHelp,
    DismissModal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
    Help,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    modal: Option<Modal>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            modal: None,
        }
    }
}

impl App {
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn modal(&self) -> Option<Modal> {
        self.modal
    }

    pub fn update(&mut self, command: Command) -> bool {
        match command {
            Command::Quit => std::mem::replace(&mut self.running, false),
            Command::ToggleHelp => {
                self.modal = if self.modal.is_some() {
                    None
                } else {
                    Some(Modal::Help)
                };
                true
            }
            Command::DismissModal => self.modal.take().is_some(),
        }
    }
}

pub async fn start(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let (tx, rx) = mpsc::channel(event::EVENT_CAPACITY);
    let (cancel, stop) = watch::channel(false);
    let input = tokio::spawn(event::pump_events(
        crossterm::event::EventStream::new(),
        tx.clone(),
        stop.clone(),
    ));
    let signals = tokio::spawn(signals::forward_shutdown(tx, stop));

    tracing::info!(version = env!("CARGO_PKG_VERSION"), "application started");
    let result = runtime::run(terminal, rx).await;
    let _ = cancel.send(true);
    let (input_result, signal_result) = tokio::join!(input, signals);
    tracing::info!(success = result.is_ok(), "application stopped");

    result?;
    input_result.map_err(io::Error::other)?;
    signal_result.map_err(io::Error::other)?;
    Ok(())
}
