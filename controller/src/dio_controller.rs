use esp_hal::gpio::{AnyPin, Level, Output, OutputConfig};
use heapless::Vec;

pub struct DioController {
    zones: Vec<Output<'static>, 6>,
}

impl DioController {
    pub fn new(zone_pins: [AnyPin; 6]) -> Self {
        let mut zones: Vec<Output, 6> = Vec::new();
        for pin in zone_pins {
            zones
                .push(Output::new(pin, Level::Low, OutputConfig::default()))
                .unwrap();
        }

        for zone in &mut zones {
            zone.set_low();
        }

        Self { zones }
    }

    pub fn status(&self) -> Vec<Level, 6> {
        let mut status = Vec::new();
        for zone in &self.zones {
            status.push(zone.output_level()).unwrap();
        }
        status
    }

    pub fn toggle_zone(&mut self, zone: usize, level: Level) -> Result<(), &str> {
        if zone > 7 || zone < 1 {
            return Err("Zone number out of range");
        }

        let zone = &mut self.zones[zone - 1];
        zone.set_level(level);
        Ok(())
    }
}
