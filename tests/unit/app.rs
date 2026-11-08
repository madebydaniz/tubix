use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tubix::{
    app::{App, Command, Modal},
    tui::input::command_for_key,
};

#[test]
fn startup_is_running_without_a_modal() {
    let app = App::default();
    assert!(app.is_running());
    assert_eq!(app.modal(), None);
}

#[test]
fn help_modal_opens_and_closes() {
    let mut app = App::default();
    assert!(app.update(Command::ToggleHelp));
    assert_eq!(app.modal(), Some(Modal::Help));
    assert!(app.update(Command::DismissModal));
    assert_eq!(app.modal(), None);
    assert!(!app.update(Command::DismissModal));
}

#[test]
fn quit_is_global_and_idempotent() {
    let mut app = App::default();
    app.update(Command::ToggleHelp);
    assert!(app.update(Command::Quit));
    assert!(!app.is_running());
    assert!(!app.update(Command::Quit));
}

#[test]
fn keyboard_maps_only_intentional_shortcuts() {
    for (code, modifiers, command) in [
        (KeyCode::Char('q'), KeyModifiers::NONE, Command::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL, Command::Quit),
        (KeyCode::Char('?'), KeyModifiers::NONE, Command::ToggleHelp),
        (KeyCode::Esc, KeyModifiers::NONE, Command::DismissModal),
    ] {
        assert_eq!(
            command_for_key(KeyEvent::new(code, modifiers)),
            Some(command)
        );
    }
    assert_eq!(
        command_for_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::ALT)),
        None
    );
    assert_eq!(
        command_for_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE)),
        None
    );
    assert_eq!(
        command_for_key(KeyEvent::new_with_kind(
            KeyCode::Char('q'),
            KeyModifiers::NONE,
            KeyEventKind::Release
        )),
        None
    );
    assert_eq!(
        command_for_key(KeyEvent::new_with_kind(
            KeyCode::Char('?'),
            KeyModifiers::NONE,
            KeyEventKind::Repeat
        )),
        None
    );
}
