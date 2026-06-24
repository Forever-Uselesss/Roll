//! Quickstart firmware for the ESP32-C6, built on `esp-hal` and the Embassy
//! async runtime.
//!
//! On boot it brings up the system clocks, hands the timer group to the Embassy
//! executor, and spawns a task that drives the board's on-board addressable RGB LED.
//! The main task then logs a heartbeat over RTT once a second.
//!
//! # LED driver
//!
//! The on-board LED is a WS2812 ("NeoPixel") on GPIO8. Rather than bit-bang the
//! protocol, the [`blink_rgb`] task uses the RMT peripheral to clock out the
//! precise pulse timing the LED expects: each color bit becomes one
//! [`PulseCode`], and a 24-bit green-red-blue frame is transmitted per update.
//! See [`encode_color`] for the bit encoding and the `T0H`/`T0L`/`T1H`/`T1L`
//! constants for the timing.
//!
//! # Logging
//!
//! Diagnostics are emitted with `defmt` over RTT; attach a probe
//! (for example `probe-rs`) to view them.
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
use esp_hal::Async;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{Channel, PulseCode, Rmt, Tx, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use panic_rtt_target as _;

/// WS2812 bit timing expressed in RMT ticks. With the RMT source clock running
/// at 80 MHz and a clock divider of 1, each tick is 12.5 ns.
///
/// The chosen pulse widths sit comfortably inside the WS2812B tolerance:
/// a "0" bit is a 0.4 us high pulse followed by a 0.85 us low pulse, and a
/// "1" bit is a 0.8 us high pulse followed by a 0.45 us low pulse.
const T0H: u16 = 32; // 0.40 us high
const T0L: u16 = 68; // 0.85 us low
const T1H: u16 = 64; // 0.80 us high
const T1L: u16 = 36; // 0.45 us low

/// One LED is 24 color bits, plus a trailing end marker to latch the line.
const PULSE_LEN: usize = 24 + 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    // Drive the on-board addressable RGB LED (WS2812) on GPIO8 via the RMT peripheral.
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80))
        .expect("failed to initialize RMT")
        .into_async();
    let led = rmt
        .channel0
        .configure_tx(&TxChannelConfig::default().with_clk_divider(1))
        .expect("failed to configure RMT channel")
        .with_pin(peripherals.GPIO8);

    spawner.spawn(blink_rgb(led).expect("failed to create LED task"));

    loop {
        info!("Hello, RISC-V Ottawa!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// The RMT pulse for a single WS2812 bit: a long high pulse for a `1`, a short
/// one for a `0`.
fn bit_pulse(one: bool) -> PulseCode {
    if one {
        PulseCode::new(Level::High, T1H, Level::Low, T1L)
    } else {
        PulseCode::new(Level::High, T0H, Level::Low, T0L)
    }
}

/// Encode an RGB color into a WS2812 pulse sequence. The LED expects its bits
/// in green, red, blue order, most-significant bit first.
fn encode_color(red: u8, green: u8, blue: u8) -> [PulseCode; PULSE_LEN] {
    let mut buf = [PulseCode::end_marker(); PULSE_LEN];
    let bits = [green, red, blue]
        .into_iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| byte & (1 << bit) != 0));
    for (slot, one) in buf.iter_mut().zip(bits) {
        *slot = bit_pulse(one);
    }
    buf
}

/// Cycle the on-board RGB LED through red, green, and blue.
#[embassy_executor::task]
async fn blink_rgb(mut led: Channel<'static, Async, Tx>) -> ! {
    const COLORS: [(u8, u8, u8); 3] = [(32, 0, 0), (0, 32, 0), (0, 0, 32)];

    loop {
        for (red, green, blue) in COLORS {
            if let Err(err) = led.transmit(&encode_color(red, green, blue)).await {
                info!("RMT transmit failed: {}", err);
            }
            // Hold the color; the line idles low, latching the LED.
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}
