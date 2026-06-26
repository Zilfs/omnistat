use crate::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn render_memory_widget(app: &App, area: Rect, buf: &mut Buffer) {
    let memory_text = Text::from(vec![
        Line::from(""),
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

    Paragraph::new(memory_text).render(area, buf);
}
