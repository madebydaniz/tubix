use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use tubix::{
    app::App,
    tui::{footer::FooterLink, input::link_for_mouse, view::render},
};

use crate::common::terminal;

fn click(column: u16, row: u16) -> MouseEvent {
    MouseEvent {
        kind: MouseEventKind::Up(MouseButton::Left),
        column,
        row,
        modifiers: KeyModifiers::NONE,
    }
}

#[test]
fn simple_click_targets_match_the_rendered_labels_after_resize() {
    for (width, height) in [(60, 12), (80, 24), (160, 30)] {
        let mut terminal = terminal(width, height);
        terminal
            .draw(|frame| render(frame, &App::default()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let footer: String = (0..width)
            .map(|x| buffer[(x, height - 1)].symbol())
            .collect();
        for (label, target) in [
            ("Donate", FooterLink::Donate),
            ("Ask Question", FooterLink::Questions),
        ] {
            let start = footer.find(label).unwrap() as u16;
            for x in start..start + label.len() as u16 {
                assert_eq!(
                    link_for_mouse(click(x, height - 1), Rect::new(0, 0, width, height), false),
                    Some(target)
                );
            }
            assert_eq!(
                link_for_mouse(
                    click(start - 1, height - 1),
                    Rect::new(0, 0, width, height),
                    false
                ),
                None
            );
            assert_eq!(
                link_for_mouse(
                    click(start + label.len() as u16, height - 1),
                    Rect::new(0, 0, width, height),
                    false
                ),
                None
            );
        }
    }
}

#[test]
fn modal_other_buttons_modifiers_and_hidden_links_cannot_open_a_browser() {
    let area = Rect::new(0, 0, 100, 25);
    let event = click(72, 24);
    assert_eq!(link_for_mouse(event, area, true), None);
    for kind in [
        MouseEventKind::Down(MouseButton::Left),
        MouseEventKind::Up(MouseButton::Right),
        MouseEventKind::Moved,
        MouseEventKind::ScrollDown,
    ] {
        assert_eq!(
            link_for_mouse(MouseEvent { kind, ..event }, area, false),
            None
        );
    }
    assert_eq!(
        link_for_mouse(
            MouseEvent {
                modifiers: KeyModifiers::CONTROL,
                ..event
            },
            area,
            false
        ),
        None
    );
    assert_eq!(link_for_mouse(click(72, 23), area, false), None);
    assert_eq!(
        link_for_mouse(click(72, 24), Rect::new(0, 0, 30, 25), false),
        None
    );
    assert_eq!(link_for_mouse(click(0, 0), Rect::default(), false), None);
}
