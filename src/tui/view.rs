use std::num::NonZeroU16;

use ratatui::{
    Frame,
    buffer::CellDiffOption,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::{App, Modal};

pub const DONATE_URL: &str = "https://github.com/sponsors/madebydaniz";
pub const QUESTIONS_URL: &str = "https://github.com/madebydaniz/tubix/discussions";

const BACKGROUND: Color = Color::Rgb(17, 20, 27);
const SURFACE: Color = Color::Rgb(23, 28, 37);
const BORDER: Color = Color::Rgb(47, 56, 70);
const MUTED: Color = Color::Rgb(144, 155, 173);
const TEXT: Color = Color::Rgb(226, 232, 240);
const ACCENT: Color = Color::Rgb(110, 231, 183);

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(BACKGROUND).fg(TEXT)),
        area,
    );
    if area.is_empty() {
        return;
    }

    let header_height = if area.height >= 10 {
        5
    } else if area.height >= 5 {
        3
    } else {
        1
    };
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(header_height),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);

    render_header(frame, header);

    let block = Block::bordered()
        .title(" DOWNLOADS ")
        .title_style(Style::default().fg(MUTED))
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(SURFACE));
    let inner = block.inner(body);
    frame.render_widget(block, body);
    let empty = centered(inner, inner.width, 5);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("↓").fg(ACCENT),
            Line::from(""),
            Line::from("No downloads yet").bold(),
            Line::from("Your queue is empty.").fg(MUTED),
            Line::from("Downloads are not available in this build.").fg(MUTED),
        ])
        .alignment(Alignment::Center),
        empty,
    );

    render_footer(frame, footer);
    if app.modal() == Some(Modal::Help) {
        render_help(frame, area);
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(BORDER));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Tubix").fg(ACCENT).bold(),
            Line::from("YouTube downloads, from your terminal.").fg(MUTED),
        ])
        .alignment(Alignment::Center),
        centered(inner, inner.width, 2),
    );
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let show_links = area.width >= 46;
    let metadata_width = version.len() as u16 + if show_links { 23 } else { 1 };
    let [hints, metadata] =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(metadata_width)]).areas(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" q ", Style::default().fg(ACCENT).bold()),
            Span::raw("quit  "),
            Span::styled("? ", Style::default().fg(ACCENT).bold()),
            Span::raw("help"),
        ])),
        hints,
    );

    let version_area = if show_links {
        let [donate, _, questions, _, version_area] = Layout::horizontal([
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(12),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .areas(metadata);
        render_link(
            frame,
            donate,
            "Donate",
            "donate",
            DONATE_URL,
            Color::Rgb(244, 114, 182),
        );
        render_link(
            frame,
            questions,
            "Ask Question",
            "questions",
            QUESTIONS_URL,
            Color::Rgb(250, 204, 21),
        );
        version_area
    } else {
        metadata
    };
    frame.render_widget(
        Paragraph::new(format!("{version} "))
            .fg(MUTED)
            .alignment(Alignment::Right),
        version_area,
    );
}

fn render_link(frame: &mut Frame, area: Rect, label: &str, id: &str, url: &str, color: Color) {
    if area.is_empty() {
        return;
    }
    // Footer labels are ASCII; balanced per-cell links keep partial redraws independent.
    for (offset, character) in label.chars().take(usize::from(area.width)).enumerate() {
        let symbol = format!("\x1b]8;id={id};{url}\x1b\\{character}\x1b]8;;\x1b\\");
        frame.buffer_mut()[(area.x + offset as u16, area.y)]
            .set_symbol(&symbol)
            .set_style(Style::default().fg(color).underlined())
            .set_diff_option(CellDiffOption::ForcedWidth(NonZeroU16::MIN));
    }
}

fn render_help(frame: &mut Frame, area: Rect) {
    let popup = centered(area, 66, 12);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(""),
            Line::from(" q / Ctrl+C    Quit Tubix"),
            Line::from(" ?             Toggle help"),
            Line::from(" Esc           Close this dialog"),
            Line::from(""),
            Line::from(format!(" Donate: {DONATE_URL}")),
            Line::from(format!(" Ask Question: {QUESTIONS_URL}")),
        ])
        .style(Style::default().fg(TEXT).bg(SURFACE))
        .block(
            Block::bordered()
                .title(" Keyboard shortcuts ")
                .border_style(Style::default().fg(ACCENT)),
        ),
        popup,
    );
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}
