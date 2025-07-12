use crate::dio_controller::DioController;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

pub type DioControllerMutex = Mutex<CriticalSectionRawMutex, DioController>;
