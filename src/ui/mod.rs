pub mod memory; // Register your sub-widget file

use crate::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
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

    // 2. Route the memory area rendering to your specialized file
    // As OmniStat grows, you will use Layout chunks here to split the screen
    // between memory::render_memory_widget, cpu::render_cpu_widget, etc.
    memory::render_memory_widget(app, inner_area, buf);
}
