use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::layout::Rect;

use super::footer::{self, FooterLink};
use crate::app::Command;

pub fn command_for_key(key: KeyEvent) -> Option<Command> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), KeyModifiers::NONE) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            Some(Command::Quit)
        }
        (KeyCode::Char('?'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Command::ToggleHelp),
        (KeyCode::Esc, KeyModifiers::NONE) => Some(Command::DismissModal),
        _ => None,
    }
}

pub fn link_for_mouse(event: MouseEvent, area: Rect, modal_open: bool) -> Option<FooterLink> {
    if modal_open
        || event.kind != MouseEventKind::Up(MouseButton::Left)
        || event.modifiers != KeyModifiers::NONE
    {
        return None;
    }
    footer::link_at(area, event.column, event.row)
}
