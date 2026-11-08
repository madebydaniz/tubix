use std::{io, time::Duration};

use crossterm::event::Event;
use ratatui::{Terminal, backend::Backend};
use tokio::{
    sync::mpsc,
    time::{Instant, sleep_until},
};

use super::{
    App,
    event::{EventMessage, EventResult},
};
use crate::tui::{input::command_for_key, view::render};

const FRAME_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / 30);

pub async fn run<B>(
    terminal: &mut Terminal<B>,
    mut events: mpsc::Receiver<EventResult>,
) -> io::Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    let mut app = App::default();
    terminal
        .draw(|frame| render(frame, &app))
        .map_err(io::Error::other)?;
    let mut dirty = false;
    let mut next_frame = Instant::now() + FRAME_INTERVAL;

    while app.is_running() {
        tokio::select! {
            biased;
            () = sleep_until(next_frame), if dirty => {
                terminal.draw(|frame| render(frame, &app)).map_err(io::Error::other)?;
                dirty = false;
                next_frame = Instant::now() + FRAME_INTERVAL;
            }
            event = events.recv() => {
                let Some(event) = event else { break };
                match event? {
                    EventMessage::Input(Event::Key(key)) => {
                        if let Some(command) = command_for_key(key) {
                            dirty |= app.update(command);
                        }
                    }
                    EventMessage::Input(Event::Resize(_, _)) | EventMessage::Redraw => dirty = true,
                    EventMessage::Shutdown => break,
                    EventMessage::Input(_) => {}
                }
            }
        }
    }
    Ok(())
}
