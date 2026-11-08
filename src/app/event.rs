use std::io;

use crossterm::event::Event;
use futures_util::{Stream, StreamExt};
use tokio::sync::{mpsc, watch};

pub const EVENT_CAPACITY: usize = 64;

#[derive(Debug)]
pub enum EventMessage {
    Input(Event),
    Redraw,
    Shutdown,
}

pub type EventResult = io::Result<EventMessage>;

pub async fn pump_events<S>(
    mut source: S,
    sender: mpsc::Sender<EventResult>,
    mut stop: watch::Receiver<bool>,
) where
    S: Stream<Item = io::Result<Event>> + Unpin,
{
    let cancelled = stop.wait_for(|stopped| *stopped);
    tokio::pin!(cancelled);

    loop {
        let next = tokio::select! {
            biased;
            _ = &mut cancelled => break,
            event = source.next() => event,
        };
        let ended = next.is_none();
        let next = next.map_or(Ok(EventMessage::Shutdown), |event| {
            event.map(EventMessage::Input)
        });
        let failed = next.is_err();
        tokio::select! {
            biased;
            _ = &mut cancelled => break,
            result = sender.send(next) => {
                if result.is_err() || failed || ended {
                    break;
                }
            }
        }
    }
}
