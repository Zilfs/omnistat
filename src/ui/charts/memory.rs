use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Bar, BarChart, Block, Sparkline, Widget},
};

use crate::App;

pub fn render_memory_chart(app: &App, area: Rect, buf: &mut Buffer) {
    let bar_width = 2;
    let bar_gap = 1;
    let space_per_bar = bar_width + bar_gap;

    let max_visible_bars = (area.width as usize) / space_per_bar;

    let history_len = app.memory_history.len();
    let skip_amount = if history_len > max_visible_bars {
        history_len - max_visible_bars
    } else {
        0
    };
    let bars: Vec<Bar> = app
        .memory_history
        .iter()
        .skip(skip_amount)
        .map(|&value| {
            Bar::default()
                .value(value)
                .style(Style::default().fg(Color::Blue))
        })
        .collect();

    let chart = BarChart::vertical(bars)
        .bar_width(bar_width as u16)
        .bar_gap(bar_gap as u16)
        .max(app.total_memory);

    chart.render(area, buf);
}
