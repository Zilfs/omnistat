use std::io;

use omnistate::{App, core::memory::MemoryStats};

fn main() -> io::Result<()> {
    let stats_memory_receiver = MemoryStats::start_tracking();

    let terminal = ratatui::init();
    let mut app = App::default();

    let result = app.run(terminal, stats_memory_receiver);

    ratatui::restore();

    result
}
