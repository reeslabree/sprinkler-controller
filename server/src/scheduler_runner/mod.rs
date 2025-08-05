pub mod runner;
pub mod spawner;

use crate::config::Config;
use crate::scheduler_runner::spawner as schedule_spawner;
use crate::types::ClientMap;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct ScheduleRunner {
    handles: Vec<(Arc<AtomicBool>, thread::JoinHandle<()>)>,
}

impl ScheduleRunner {
    pub fn new(config: Config, clients: &ClientMap) -> Self {
        let handles =
            schedule_spawner::spawn(config.schedules.clone(), config.stagger_zones, clients);

        Self { handles }
    }

    pub fn update(&mut self, config: Config, clients: &ClientMap) -> Self {
        for (running, handle) in self.handles.drain(..) {
            running.store(false, Ordering::Relaxed);
            handle.join().unwrap();
        }

        let handles =
            schedule_spawner::spawn(config.schedules.clone(), config.stagger_zones, clients);

        Self { handles }
    }
}
