use crate::utils::convert::bytes_to_mb;

use std::{
    sync::mpsc::{Receiver, channel},
    thread::{self, sleep},
    time::Duration,
};

use sysinfo::System;

pub struct MemoryStats {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

impl MemoryStats {
    fn fetch(system: &mut System) -> Self {
        system.refresh_all();

        MemoryStats {
            total_memory: system.total_memory(),
            used_memory: system.used_memory(),
            free_memory: system.free_memory(),
        }
    }

    pub fn start_tracking() -> Receiver<Self> {
        let (tx, rx) = channel();

        let mut system = System::new();

        thread::spawn(move || {
            loop {
                let stats = Self::fetch(&mut system);

                println!(
                    "Memory usage: {} MB / {} MB (free {} MB)",
                    bytes_to_mb(stats.used_memory),
                    bytes_to_mb(stats.total_memory),
                    bytes_to_mb(stats.free_memory)
                );

                if tx.send(stats).is_err() {
                    break;
                }

                sleep(Duration::from_secs(3));
            }
        });

        rx
    }
}
