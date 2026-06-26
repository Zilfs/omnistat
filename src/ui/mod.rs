pub mod charts;
pub mod memory;

use crate::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Widget},
};

/// The main entry point for rendering the entire application layout
pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    // 1. Draw global outer shell instructions
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let global_block = Block::new().title_bottom(instructions.centered());

    let inner_area = global_block.inner(area);
    global_block.render(area, buf);

    let current_usage = app.memory_history.last().unwrap_or(&0);
    let title = format!(" Memory Usage History (Current: {} MB) ", current_usage);

    let memory_block = Block::bordered()
        .title(title.bold())
        .border_style(Style::default().fg(Color::Cyan));

    let memory_inner_area = memory_block.inner(inner_area);
    memory_block.render(inner_area, buf);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .flex(ratatui::layout::Flex::Start)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(memory_inner_area);

    let chart_area = Layout::default()
        .direction(Direction::Horizontal)
        .flex(ratatui::layout::Flex::Start)
        .constraints([Constraint::Length(50)])
        .split(chunks[0])[0];

    charts::memory::render_memory_chart(app, chart_area, buf);
    memory::render_memory_widget(app, chunks[0], buf);
}
