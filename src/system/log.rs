// Logging utility.
//
// Features:
// - `log_defmt`: log via defmt
// - `log_uart`: log via UART (not yet implemented)

#[cfg(feature = "log_defmt")]
defmt::timestamp!(
    "{=u64:us}",
    esp_hal::time::Instant::now()
        .duration_since_epoch()
        .as_micros()
);

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(feature = "log_defmt")]
        defmt::info!($($arg)*);
    };
}

pub use log_info;

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "log_defmt")]
        defmt::warn!($($arg)*);
    };
}

pub use log_warn;

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(feature = "log_defmt")]
        defmt::error!($($arg)*);
    };
}

pub use log_error;

// TODO: implement UART logging (`log_uart`)
