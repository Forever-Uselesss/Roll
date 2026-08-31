#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
              holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]



use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    Config,
    clock::CpuClock,
    timer::timg::TimerGroup,
    interrupt::software::SoftwareInterruptControl
};
use roll::log::{
    log_info,
    log_warn,
    log_error
};
use panic_rtt_target as _;

// Inject the required ESP-IDF application descriptor macro here:
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // optional logging with defmt.
    #[cfg(feature = "log_defmt")]
    rtt_target::rtt_init_defmt!();

    // fist log is important
    log_info!("hello world");
    log_warn!("hello world");
    log_error!("hello world");


    // init code for ESP32-C6
    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);

    log_info!("ESP32-C6 initialized successfully!");
    // Embedded main functions must NEVER return.
    // They must loop infinitely.
    loop {
        // Your application loop
        log_info!("tick...");
        Timer::after(Duration::from_secs(1)).await;

        log_warn!("tick...");
        Timer::after(Duration::from_secs(2)).await;

        log_error!("tick...");
        Timer::after(Duration::from_secs(3)).await;

        // Timer::after(Duration::from_secs(1)).await;
    }
}
