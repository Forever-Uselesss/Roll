# Support

> If you are looking to get started please refer to [Quick Start Guide](https://github.com/Forever-Uselesss/Roll/tree/main/support/documentation/quick_start.md). 

## Table of Contents
- [Get Support](#get-support)
    - [documentation](#documentation)
    - [hardware](#hardware)
- [Need Help?](#need-help)
- [Progress](#progress)

## Get Support

>This section contains useful resources for developing Roll, including Rust, embedded development, ESP32 documentation, and community support channels.

Let me start by offering gratitude.
Embedded development can involve several interacting toolchains, targets, debuggers, and hardware-specific dependencies. Adding a new ecosystem can make the initial setup challenging. These documents aim to make the development environment easier to understand and reproduce.

- [Rust Book](https://doc.rust-lang.org/stable/book/)
- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [Espressif Rust Book](https://docs.espressif.com/projects/rust/book/)
    - [ESP-rs documentation and resources](https://docs.espressif.com/projects/rust/)
    - [ESP-rs ESP32-C6 HAL (v1.1.1)](https://docs.espressif.com/projects/rust/esp-hal/1.1.1/esp32c6/esp_hal/index.html)
    - [ESP-rs examples](https://github.com/esp-rs/esp-hal/tree/main/examples)
    - [ESP-IDF ESP32-C6](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/)

    
### Documentation

This folder contains all documentation required for the project. 

### Hardware

This folder contains all documentation required for the project.

---
## Need Help?
If none of the above documents solved your problem, please open an issue on the GitHub repository. Be sure to include:

1.  **Reproduction Steps:** The exact sequence of actions that caused the issue.
2.  **Expected Behavior:** What *should* have happened.
3.  **Actual Behavior:** What *did* happen (including log outputs!).

## Progress

### Development Status
>Currently using an ESP32-C6 development board. The goal is to develop a fully standalone embedded platform after prototyping.

- **Designed** → Architecture and requirements are defined.
- **Implemented** → The required code or hardware exists.
- **Validated** → The implementation has been tested and confirmed working on actual hardware.


- [ ] Define requirements
- [ ] Define architecture
- [x] Select MCU
- [x] Establish Rust toolchain
- [x] Establish probe-rs debugging
- [x] Establish defmt logging
- [ ] Validate a functional prototype

### Peripherals
- [ ] Input device
- [ ] ADXL345
- [ ] audio
- [ ] display
- [ ] feedback
- [ ] Other devices

### Hardware
- [ ] Design prototype PCB
- [ ] Manufacture prototype PCB
- [ ] Bring up prototype PCB
- [ ] Validate hardware

### Integration
- [ ] Integrate input hardware with firmware
- [ ] Validate complete dice-rolling workflow
