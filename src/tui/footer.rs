use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

pub const DONATE_URL: &str = "https://github.com/sponsors/madebydaniz";
pub const QUESTIONS_URL: &str = "https://github.com/madebydaniz/tubix/discussions";

const DONATE_COLOR: Color = Color::Rgb(244, 167, 185);
const QUESTION_COLOR: Color = Color::Rgb(110, 231, 183);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FooterLink {
    Donate,
    Questions,
}

impl FooterLink {
    pub fn url(self) -> &'static str {
        match self {
            Self::Donate => DONATE_URL,
            Self::Questions => QUESTIONS_URL,
        }
    }
}

struct FooterLayout {
    hints: Rect,
    donate: Rect,
    questions: Rect,
    version: Rect,
}

fn layout(area: Rect) -> FooterLayout {
    let version_width = env!("CARGO_PKG_VERSION").len() as u16 + 2;
    let show_links = area.width >= 14 + 22 + version_width;
    let metadata_width = version_width + if show_links { 22 } else { 0 };
    let [hints, metadata] =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(metadata_width)]).areas(area);

    let (donate, questions, version) = if show_links {
        let [donate, _, questions, _, version] = Layout::horizontal([
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(12),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .areas(metadata);
        (donate, questions, version)
    } else {
        (Rect::default(), Rect::default(), metadata)
    };
    FooterLayout {
        hints,
        donate,
        questions,
        version,
    }
}

pub(crate) fn render(frame: &mut Frame, area: Rect) {
    let regions = layout(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" q ", Style::default().fg(Color::Rgb(110, 231, 183)).bold()),
            Span::raw("quit  "),
            Span::styled("? ", Style::default().fg(Color::Rgb(110, 231, 183)).bold()),
            Span::raw("help"),
        ])),
        regions.hints,
    );
    frame.render_widget(
        Paragraph::new("Donate").fg(DONATE_COLOR).underlined(),
        regions.donate,
    );
    frame.render_widget(
        Paragraph::new("Ask Question")
            .fg(QUESTION_COLOR)
            .underlined(),
        regions.questions,
    );
    frame.render_widget(
        Paragraph::new(format!("v{} ", env!("CARGO_PKG_VERSION")))
            .fg(Color::Rgb(144, 155, 173))
            .alignment(Alignment::Right),
        regions.version,
    );
}

pub(crate) fn link_at(area: Rect, column: u16, row: u16) -> Option<FooterLink> {
    if area.is_empty() {
        return None;
    }
    let regions = layout(Rect::new(area.x, area.bottom() - 1, area.width, 1));
    let position = Position::new(column, row);
    if regions.donate.contains(position) {
        Some(FooterLink::Donate)
    } else if regions.questions.contains(position) {
        Some(FooterLink::Questions)
    } else {
        None
    }
}
