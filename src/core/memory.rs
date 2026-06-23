use std::{thread::sleep, time::Duration};

use sysinfo::System;

pub fn track_memory_usage() {
    let mut system = System::new();

    loop {
        system.refresh_all();

        let total_memory = system.total_memory() / 1024 / 1024;
        let used_memory = system.used_memory() / 1024 / 1024;

        println!("Memory usage: {} MB / {} MB", used_memory, total_memory);

        sleep(Duration::from_secs(3));
    }
}
