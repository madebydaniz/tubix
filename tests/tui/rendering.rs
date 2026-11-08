use crate::common::{screen, terminal};
use tubix::{
    app::{App, Command},
    tui::view::{DONATE_URL, QUESTIONS_URL, render},
};

#[test]
fn dashboard_shows_brand_empty_state_links_and_version() {
    let mut terminal = terminal(160, 30);
    terminal
        .draw(|frame| render(frame, &App::default()))
        .unwrap();
    let output = screen(&terminal);
    for expected in [
        "Tubix",
        "No downloads yet",
        "q",
        "Donate",
        "Ask Question",
        DONATE_URL,
        QUESTIONS_URL,
        env!("CARGO_PKG_VERSION"),
    ] {
        assert!(output.contains(expected), "missing {expected}");
    }
}

#[test]
fn small_terminals_render_without_panics() {
    for width in [0, 1, 2, 10, 30, 60, 100] {
        for height in [0, 1, 2, 3, 5, 12] {
            let mut terminal = terminal(width, height);
            let mut app = App::default();
            terminal.draw(|frame| render(frame, &app)).unwrap();
            app.update(Command::ToggleHelp);
            terminal.draw(|frame| render(frame, &app)).unwrap();
        }
    }
}

#[test]
fn help_modal_renders_shortcuts_and_can_be_dismissed() {
    let mut terminal = terminal(100, 25);
    let mut app = App::default();
    app.update(Command::ToggleHelp);
    terminal.draw(|frame| render(frame, &app)).unwrap();
    assert!(screen(&terminal).contains("Keyboard shortcuts"));
    app.update(Command::DismissModal);
    terminal.draw(|frame| render(frame, &app)).unwrap();
    assert!(!screen(&terminal).contains("Keyboard shortcuts"));
}
