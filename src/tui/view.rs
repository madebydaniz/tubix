use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

use super::footer;
use crate::app::{App, Modal};

pub use super::footer::{DONATE_URL, QUESTIONS_URL};

const BACKGROUND: Color = Color::Reset;
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

    let [header, body, footer_area] = Layout::vertical([
        Constraint::Length(if area.height >= 5 { 3 } else { 1 }),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);
    render_header(frame, header);

    let block = Block::bordered()
        .title(" DOWNLOADS ")
        .title_style(Style::default().fg(MUTED))
        .border_style(Style::default().fg(BORDER));
    let inner = block.inner(body);
    frame.render_widget(block, body);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("↓").fg(ACCENT),
            Line::from(""),
            Line::from("No downloads yet").bold(),
            Line::from("Your queue is empty.").fg(MUTED),
            Line::from(
                app.notice()
                    .unwrap_or("Downloads are not available in this build."),
            )
            .fg(MUTED),
        ])
        .alignment(Alignment::Center),
        centered(inner, inner.width, 5),
    );
    footer::render(frame, footer_area);
    if app.modal() == Some(Modal::Help) {
        render_help(frame, area);
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" Tubix ", Style::default().fg(ACCENT).bold()),
            Span::styled(
                " YouTube downloads, from your terminal.",
                Style::default().fg(MUTED),
            ),
        ])),
        centered(area, area.width, 1),
    );
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
            Line::from(" Click Donate or Ask Question to open your browser."),
            Line::from(""),
            Line::from(format!(" Donate: {DONATE_URL}")),
            Line::from(format!(" Ask Question: {QUESTIONS_URL}")),
        ])
        .style(Style::default().fg(TEXT).bg(BACKGROUND))
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
