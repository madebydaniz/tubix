use std::num::NonZeroU16;

use ratatui::buffer::CellDiffOption;

use crate::common::{screen, terminal, visible_symbol};
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
        env!("CARGO_PKG_VERSION"),
    ] {
        assert!(output.contains(expected), "missing {expected}");
    }
    assert!(!output.contains(DONATE_URL));
    assert!(!output.contains(QUESTIONS_URL));
    assert!(!output.contains("your download workspace"));
}

#[test]
fn header_centers_the_brand_and_slogan_with_vertical_padding() {
    for width in [80, 100, 160] {
        let mut terminal = terminal(width, 25);
        terminal
            .draw(|frame| render(frame, &App::default()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let row = |y| {
            (0..width)
                .map(|x| visible_symbol(buffer[(x, y)].symbol()))
                .collect::<String>()
        };
        assert!(row(0).trim().is_empty());
        let title_x = row(1).find("Tubix").unwrap();
        assert!(title_x.abs_diff(usize::from(width) - title_x - 5) <= 1);
        let slogan = "YouTube downloads, from your terminal.";
        let slogan_x = row(2).find(slogan).unwrap();
        assert!(slogan_x.abs_diff(usize::from(width) - slogan_x - slogan.len()) <= 1);
        assert!(row(3).trim().is_empty());
    }
}

#[test]
fn footer_keeps_hints_links_and_version_on_the_last_row() {
    for width in [60, 80, 160] {
        let mut terminal = terminal(width, 25);
        terminal
            .draw(|frame| render(frame, &App::default()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let footer: String = (0..width)
            .map(|x| visible_symbol(buffer[(x, 24)].symbol()))
            .collect();
        assert!(footer.starts_with(" q quit  ? help"));
        assert!(footer.trim_end().ends_with(&format!(
            "Donate  Ask Question  v{}",
            env!("CARGO_PKG_VERSION")
        )));
    }
}

#[test]
fn footer_links_have_balanced_osc8_sequences_and_fixed_cell_widths() {
    let mut terminal = terminal(100, 25);
    terminal
        .draw(|frame| render(frame, &App::default()))
        .unwrap();
    let buffer = terminal.backend().buffer();
    for (id, url, label) in [
        ("donate", DONATE_URL, "Donate"),
        ("questions", QUESTIONS_URL, "Ask Question"),
    ] {
        let cells: Vec<_> = buffer
            .content
            .iter()
            .filter(|cell| cell.symbol().contains(url))
            .collect();
        assert_eq!(cells.len(), label.len());
        for (cell, character) in cells.iter().zip(label.chars()) {
            assert_eq!(
                cell.symbol(),
                format!("\x1b]8;id={id};{url}\x1b\\{character}\x1b]8;;\x1b\\")
            );
            assert_eq!(
                cell.diff_option,
                CellDiffOption::ForcedWidth(NonZeroU16::MIN)
            );
        }
    }
}

#[test]
fn footer_links_survive_modal_overlap_and_resize() {
    let mut terminal = terminal(100, 12);
    let mut app = App::default();
    terminal.draw(|frame| render(frame, &app)).unwrap();
    let initial = terminal.backend().buffer().clone();
    app.update(Command::ToggleHelp);
    terminal.draw(|frame| render(frame, &app)).unwrap();
    app.update(Command::DismissModal);
    terminal.draw(|frame| render(frame, &app)).unwrap();
    assert_eq!(terminal.backend().buffer(), &initial);
    terminal.backend_mut().resize(60, 12);
    terminal.draw(|frame| render(frame, &app)).unwrap();
    let output = screen(&terminal);
    assert!(output.contains("Donate  Ask Question"));
    assert!(!output.contains(DONATE_URL));
    assert!(!output.contains(QUESTIONS_URL));
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
