// src/log.rs

#[cfg(feature = "defmt")]
defmt::timestamp!(
    "{=u64:us}",
    esp_hal::time::Instant::now()
        .duration_since_epoch()
        .as_micros()
);

#[cfg(feature = "defmt")]
macro_rules! log_info {
    ($($arg:tt)*) => {
        defmt::info!($($arg)*);
    };
}

#[cfg(not(feature = "defmt"))]
macro_rules! log_info {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "defmt")]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        defmt::warn!($($arg)*);
    };
}

#[cfg(not(feature = "defmt"))]
macro_rules! log_warn {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "defmt")]
macro_rules! log_error {
    ($($arg:tt)*) => {
        defmt::error!($($arg)*);
    };
}

#[cfg(not(feature = "defmt"))]
macro_rules! log_error {
    ($($arg:tt)*) => {};
}

pub(crate) use log_error;
pub(crate) use log_info;
pub(crate) use log_warn;
