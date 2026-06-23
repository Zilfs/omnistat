use crate::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn render_memory_widget(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Memory Dashboard ".bold());
    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::THICK);

    let memory_text = Text::from(vec![
        Line::from(vec![
            "Used Memory:  ".into(),
            format!("{} MB", app.used_memory).yellow().bold(),
        ]),
        Line::from(vec![
            "Total Memory: ".into(),
            format!("{} MB", app.total_memory).blue(),
        ]),
        Line::from(vec![
            "Free Memory:  ".into(),
            format!("{} MB", app.free_memory).green(),
        ]),
    ]);

    Paragraph::new(memory_text)
        .left_aligned()
        .block(block)
        .render(area, buf);
}
