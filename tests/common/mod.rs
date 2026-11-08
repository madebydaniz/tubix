use ratatui::{Terminal, backend::TestBackend};

pub fn terminal(width: u16, height: u16) -> Terminal<TestBackend> {
    Terminal::new(TestBackend::new(width, height)).unwrap()
}

pub fn screen(terminal: &Terminal<TestBackend>) -> String {
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| visible_symbol(cell.symbol()))
        .collect()
}

pub fn visible_symbol(symbol: &str) -> &str {
    if symbol.starts_with("\x1b]8;") {
        symbol
            .split_once("\x1b\\")
            .and_then(|(_, text)| text.strip_suffix("\x1b]8;;\x1b\\"))
            .unwrap_or(symbol)
    } else {
        symbol
    }
}
