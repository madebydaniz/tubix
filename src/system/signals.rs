use std::io;

use tokio::sync::{mpsc, watch};

use crate::app::event::{EventMessage, EventResult};

pub(crate) async fn forward_shutdown(
    sender: mpsc::Sender<EventResult>,
    mut stop: watch::Receiver<bool>,
) {
    let cancelled = stop.wait_for(|stopped| *stopped);
    tokio::pin!(cancelled);
    let result = tokio::select! {
        biased;
        _ = &mut cancelled => return,
        result = shutdown_signal() => result.map(|()| EventMessage::Shutdown),
    };
    tokio::select! {
        biased;
        _ = &mut cancelled => {},
        _ = sender.send(result) => {},
    }
}

async fn shutdown_signal() -> io::Result<()> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await
}
