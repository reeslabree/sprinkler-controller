use embassy_time::Instant;
use esp_storage::FlashStorage;

pub struct Storage {
    storage: FlashStorage,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            storage: FlashStorage::new(),
        }
    }

    pub fn set_schedule(&mut self) {
        Durat
    }
}
