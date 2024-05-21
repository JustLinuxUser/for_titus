use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn floating_window(size: Rect) -> Rect {
    let hor_float = Layout::default()
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .direction(Direction::Horizontal)
        .split(size)[1];
    let floating = Layout::default()
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .direction(Direction::Vertical)
        .split(hor_float)[1];
    floating
}
