use ratatui::{
    Frame,
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

    let footer_height = if area.width >= 66 && area.height >= 10 {
        3
    } else {
        1
    };
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(footer_height),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" Tubix ", Style::default().fg(ACCENT).bold()),
            Span::styled(" / your download workspace", Style::default().fg(MUTED)),
        ]))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER)),
        ),
        header,
    );

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

fn render_footer(frame: &mut Frame, area: Rect) {
    let [hints, metadata] =
        Layout::horizontal([Constraint::Length(22), Constraint::Min(0)]).areas(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" q ", Style::default().fg(ACCENT).bold()),
            Span::raw("quit  "),
            Span::styled("? ", Style::default().fg(ACCENT).bold()),
            Span::raw("help"),
        ])),
        hints,
    );

    let lines = if area.height >= 3 {
        vec![
            Line::from(format!("v{} ", env!("CARGO_PKG_VERSION"))),
            Line::from(format!("Donate: {DONATE_URL} ")),
            Line::from(format!("Ask Question: {QUESTIONS_URL} ")),
        ]
    } else {
        vec![Line::from(format!(
            "Donate · Ask Question · v{} ",
            env!("CARGO_PKG_VERSION")
        ))]
    };
    frame.render_widget(
        Paragraph::new(lines).fg(MUTED).alignment(Alignment::Right),
        metadata,
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
