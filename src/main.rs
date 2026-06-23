use omnistate::core::memory::MemoryStats;

fn main() {
    let _stats_memory_receiver = MemoryStats::start_tracking();

    loop {
        std::thread::park();
    }
}
