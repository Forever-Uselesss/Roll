#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
              holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{Channel, PulseCode, Rmt, Tx, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::Async;
use panic_rtt_target as _;

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // 1. Initialize your log system (if your HAL template requires it,
    // or probe-rs will catch defmt automatically via RTT)
    info!("hello world");

    // 2. Embedded main functions must NEVER return.
    // They must loop infinitely.
    loop {
        // Your application loop
    }
}
