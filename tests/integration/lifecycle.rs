use crate::common::{screen, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use futures_util::stream;
use std::{io, time::Duration};
use tokio::sync::{mpsc, watch};
use tubix::app::{
    event::{EventMessage, pump_events},
    runtime::run,
};

#[tokio::test]
async fn startup_draws_and_q_shuts_down() {
    let (tx, rx) = mpsc::channel(4);
    tx.send(Ok(EventMessage::Input(Event::Key(KeyEvent::new(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
    )))))
    .await
    .unwrap();
    let mut terminal = terminal(100, 25);
    run(&mut terminal, rx).await.unwrap();
    assert!(screen(&terminal).contains("Tubix"));
}

#[tokio::test]
async fn closed_event_channel_exits_cleanly() {
    let (tx, rx) = mpsc::channel(1);
    drop(tx);
    run(&mut terminal(80, 24), rx).await.unwrap();
}

#[tokio::test]
async fn event_errors_reach_the_caller() {
    let (tx, rx) = mpsc::channel(1);
    tx.send(Err(io::Error::other("input failed")))
        .await
        .unwrap();
    let error = run(&mut terminal(80, 24), rx).await.unwrap_err();
    assert_eq!(error.to_string(), "input failed");
}

#[tokio::test]
async fn input_pump_forwards_events_and_stops_at_eof() {
    let (tx, mut rx) = mpsc::channel(4);
    let (_cancel, stop) = watch::channel(false);
    let input = stream::iter([Ok(Event::Resize(100, 25))]);
    pump_events(input, tx, stop).await;
    assert!(matches!(
        rx.recv().await,
        Some(Ok(EventMessage::Input(Event::Resize(100, 25))))
    ));
    assert!(matches!(rx.recv().await, Some(Ok(EventMessage::Shutdown))));
    assert!(rx.recv().await.is_none());
}

#[tokio::test]
async fn cancellation_interrupts_a_full_channel() {
    let (tx, _rx) = mpsc::channel(1);
    tx.send(Ok(EventMessage::Redraw)).await.unwrap();
    let (cancel, stop) = watch::channel(false);
    let input = stream::iter([Ok(Event::Resize(100, 25))]);
    let task = tokio::spawn(pump_events(input, tx, stop));
    tokio::task::yield_now().await;
    cancel.send(true).unwrap();
    tokio::time::timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test(start_paused = true)]
async fn input_eof_shuts_down_even_when_other_senders_exist() {
    let (tx, rx) = mpsc::channel(4);
    let other_sender = tx.clone();
    let (_cancel, stop) = watch::channel(false);
    let input = tokio::spawn(pump_events(stream::empty(), tx, stop));
    let mut terminal = terminal(80, 24);
    tokio::time::timeout(Duration::from_secs(1), run(&mut terminal, rx))
        .await
        .expect("input EOF must shut down the application")
        .unwrap();
    input.await.unwrap();
    drop(other_sender);
}

#[tokio::test]
async fn input_errors_are_forwarded_before_the_pump_stops() {
    let (tx, mut rx) = mpsc::channel(4);
    let (_cancel, stop) = watch::channel(false);
    pump_events(
        stream::iter([Err(io::Error::other("read failed"))]),
        tx,
        stop,
    )
    .await;
    assert_eq!(
        rx.recv().await.unwrap().unwrap_err().to_string(),
        "read failed"
    );
    assert!(rx.recv().await.is_none());
}

#[tokio::test]
async fn precancelled_input_does_not_forward_events() {
    let (tx, mut rx) = mpsc::channel(4);
    let (_cancel, stop) = watch::channel(true);
    pump_events(stream::iter([Ok(Event::Resize(80, 24))]), tx, stop).await;
    assert!(rx.recv().await.is_none());
}

#[tokio::test(start_paused = true)]
async fn active_redraw_requests_are_coalesced_to_the_frame_limit() {
    let (tx, rx) = mpsc::channel(64);
    let producer = tokio::spawn(async move {
        for _ in 0..50 {
            tx.send(Ok(EventMessage::Redraw)).await.unwrap();
        }
        tokio::time::sleep(Duration::from_millis(34)).await;
        tx.send(Ok(EventMessage::Shutdown)).await.unwrap();
    });
    let mut terminal = terminal(80, 24);
    run(&mut terminal, rx).await.unwrap();
    producer.await.unwrap();
    assert_eq!(terminal.get_frame().count(), 2);
}

#[tokio::test(start_paused = true)]
async fn idle_renderer_does_not_redraw() {
    let (tx, rx) = mpsc::channel(4);
    let mut terminal = terminal(80, 24);
    {
        let runner = run(&mut terminal, rx);
        tokio::pin!(runner);
        tokio::select! {
            result = &mut runner => panic!("runner stopped early: {result:?}"),
            () = tokio::time::sleep(Duration::from_secs(60)) => {}
        }
        tx.send(Ok(EventMessage::Shutdown)).await.unwrap();
        runner.await.unwrap();
    }
    assert_eq!(terminal.get_frame().count(), 1);
}
