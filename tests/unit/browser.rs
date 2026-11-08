use std::{process::Stdio, time::Duration};

use tokio::{process::Command, time::Instant};

use super::wait_for_launch;

#[tokio::test(start_paused = true)]
async fn a_long_lived_launcher_does_not_block_later_actions() {
    let mut child = Command::new("/bin/cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let input = child.stdin.take().unwrap();
    let started = Instant::now();
    wait_for_launch(child).await.unwrap();
    assert_eq!(started.elapsed(), Duration::from_millis(250));
    drop(input);
}

#[tokio::test]
async fn an_immediate_launcher_failure_reaches_the_application() {
    let mut child = Command::new("/bin/sh")
        .args(["-c", "exit 7"])
        .spawn()
        .unwrap();
    assert!(!child.wait().await.unwrap().success());
    assert!(wait_for_launch(child).await.is_err());
}
