# ROLL
>Designed in collaboration with [RISC-V Ottawa (RVO)](https://riscvottawa.ca/)


An open-source, accessible electronic dice roller built on RISC-V architecture. Designed in collaboration with [RISC-V Ottawa (RVO)](https://riscvottawa.ca/), this project removes physical barriers to tabletop gaming so everyone can roll D20s regardless of physical ability.

## About the Project

Physical tabletop games such as D&D, Pathfinder, and classic board games rely heavily on physical dice rolls. For many players with motor disabilities or other physical challenges, rolling traditional dice can be a significant barrier.

It is frustrating that accessibility tools can be expensive and often assume that every user can interact with them in the same way, leaving a subset of users overlooked. This is compounded by an accessibility market where relatively simple assistive tools can become disproportionately expensive. ROLL was born from that frustration: the desire for a simple, accessible device that should not need to be complicated or prohibitively expensive to exist. 

ROLL boasts extensive customization, putting control in the user's hands and allowing the device to be adapted to individual needs regardless of physical ability.
* **Proposal Details:** View the original project proposal on [MakerRepo](https://makerepo.com/project_proposals/576).

## Getting Started


Getting ROLL running is straightforward.


### Prerequisites

* Hardware: An ESP32-C6 development board is recommended for starting development.
* Software: Rust toolchain (rustup) installed.
* Guidance: Please consult our [Support Documentation](https://github.com/Forever-Uselesss/Roll/tree/main/support) Documentation for detailed setup instructions.
* Patience

### Workflow Quick Summary

1. Clone: Clone the repository.

```` bash
    git clone <repo-url>
    cd Roll
````
2. Build: Compile the firmware.

```` bash
    cargo build --profile dev --all-features
````
3. Test: Run the included unit tests to ensure system integrity.

```` bash
    cargo test
````

## Repository Structure

| Path       | Description                        |
| ---------- | ---------------------------------- |
| `src/`     | Firmware source                    |
| `tests/`   | Tests                              |
| `support/` | Project documentation and support  |
| `.cargo/`  | Cargo and target configuration     |
| `.github/` | CI workflows                       |


## Accessibility

DICE is designed around the principle that tabletop gaming should be accessible to everyone.

The device is intended to reduce the physical barriers associated with traditional dice while allowing users to customize its operation to their individual needs.

Accessibility is an ongoing part of the project's development. Feedback from users with different physical abilities is especially valuable.

## Contributing

Contributions are welcome! Whether you're interested in firmware, hardware, accessibility, documentation, or testing, there are many ways to contribute.

Before opening an issue or pull request, please review the [Support](https://github.com/Forever-Uselesss/Roll/tree/main/support) documentation and [`CONTRIBUTING`](CONTRIBUTING)

## License

DICE is open-source software. No warranty or gurantees provided. See [`LICENSE`](LICENSE) for details.
