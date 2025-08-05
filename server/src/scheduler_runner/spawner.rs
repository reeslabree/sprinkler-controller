use crate::scheduler_runner::runner as schedule_runner;
use crate::types::{ClientMap, Day, Schedule};

use chrono::{Datelike, Local, Timelike};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

const THREAD_POLL_MILLIS: u64 = 1000;

pub(super) fn spawn(
    schedules: Vec<Schedule>,
    stagger_zones: bool,
    clients: &ClientMap,
) -> Vec<(Arc<AtomicBool>, thread::JoinHandle<()>)> {
    schedules
        .iter()
        .map(|schedule| {
            let running = Arc::new(AtomicBool::new(true));
            let thread_running = running.clone();

            let schedule = schedule.clone();
            let start_time = schedule.start_time_minutes;
            let days = schedule.days.clone();

            let clients = clients.clone();

            let handle = thread::spawn(move || {
                loop {
                    if !thread_running.load(Ordering::Relaxed) {
                        break;
                    }

                    let now = Local::now();
                    let current_day: Day = now.weekday().into();

                    let current_time_minutes = now.hour() * 60 + now.minute();

                    if days.contains(&current_day) && current_time_minutes == start_time {
                        let _ = schedule_runner::run(schedule.clone(), stagger_zones, &clients);
                    }

                    thread::sleep(Duration::from_millis(THREAD_POLL_MILLIS));
                }
            });

            (running, handle)
        })
        .collect()
}
