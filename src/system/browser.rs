use std::{io, process::Stdio, time::Duration};

use tokio::process::Command;

pub(crate) async fn open(url: &'static str) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("rundll32.exe");
        command.arg("url.dll,FileProtocolHandler");
        command
    };
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let mut command = Command::new("xdg-open");

    let child = command
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(false)
        .spawn()?;

    wait_for_launch(child).await
}

async fn wait_for_launch(mut child: tokio::process::Child) -> io::Result<()> {
    let status = match tokio::time::timeout(Duration::from_millis(250), child.wait()).await {
        Ok(status) => status?,
        Err(_) => {
            // Some desktop launchers remain alive until the browser window closes.
            tokio::spawn(async move {
                if let Err(error) = child.wait().await {
                    tracing::warn!(%error, "could not reap browser launcher");
                }
            });
            return Ok(());
        }
    };
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "browser launcher exited with {status}"
        )))
    }
}

#[cfg(all(test, unix))]
#[path = "../../tests/unit/browser.rs"]
mod tests;
