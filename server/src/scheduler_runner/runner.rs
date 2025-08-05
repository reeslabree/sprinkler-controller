use std::thread;
use std::time::Duration;

use shared::{ServerMessage, ToggleZonePayload};

use crate::error::ServerError;
use crate::message::send_to_controller;
use crate::types::{ClientMap, Schedule};

const ZONE_STAGGER_DURATION_SECS: u64 = 10;

pub(super) async fn run(
    schedule: Schedule,
    stagger_zones: bool,
    clients: &ClientMap,
) -> Result<(), ServerError> {
    let active_periods_len = schedule.active_periods.len();
    for (index, period) in schedule.active_periods.iter().enumerate() {
        let duration_secs: u64 = (period.duration_minutes as u64) * 60;
        let zone = period.zone;

        // If first zone, turn it on
        if index == 0 {
            send_to_controller(
                clients,
                &serde_json::to_string(&ServerMessage::ToggleZone(ToggleZonePayload {
                    activate: true,
                    zone: zone.into(),
                }))
                .unwrap(),
            )
            .await;
        }

        // sleep while it runs
        if stagger_zones {
            thread::sleep(Duration::from_secs(
                duration_secs - ZONE_STAGGER_DURATION_SECS,
            ));
        } else {
            thread::sleep(Duration::from_secs(duration_secs));
        }

        // if not last zone, turn on next zone
        if index < active_periods_len - 1 {
            let next_zone = schedule.active_periods.iter().nth(index + 1).unwrap().zone;

            send_to_controller(
                clients,
                &serde_json::to_string(&ServerMessage::ToggleZone(ToggleZonePayload {
                    activate: true,
                    zone: next_zone.into(),
                }))
                .unwrap(),
            )
            .await;
        }

        // if staggering, let them run together for a bit
        if stagger_zones {
            thread::sleep(Duration::from_secs(ZONE_STAGGER_DURATION_SECS));
        }

        // turn off current zone
        send_to_controller(
            clients,
            &serde_json::to_string(&ServerMessage::ToggleZone(ToggleZonePayload {
                activate: false,
                zone: zone.into(),
            }))
            .unwrap(),
        )
        .await;
    }

    Ok(())
}
