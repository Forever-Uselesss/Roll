# DICE

An open-source, accessible electronic dice roller built on RISC-V architecture. Designed in collaboration with [RISC-V Ottawa (RVO)](https://riscvottawa.ca/), this project removes physical barriers to tabletop gaming so everyone can roll D20s regardless of physical ability.

## About the Project

Physical tabletop games such as D&D, Pathfinder, and classic board games rely heavily on physical dice rolls. For many players with motor disabilities or other physical challenges, rolling traditional dice can be a significant barrier.

It is frustrating that accessibility tools can be expensive and often assume that every user can interact with them in the same way, leaving a subset of users overlooked. This is compounded by an accessibility market where relatively simple assistive tools can become disproportionately expensive. DICE was born from that frustration: the desire for a simple, accessible device that should not need to be complicated or prohibitively expensive to exist. 

DICE boasts extensive customization, putting control in the user's hands and allowing the device to be adapted to individual needs regardless of physical ability.
* **Proposal Details:** View the original project proposal on [MakerRepo](https://makerepo.com/project_proposals/576).

## Getting Started

### Prerequisites

* Rust toolchain
* ESP32-C6 development board

  * If using a different development board, update the appropriate configuration files.
* Patience
* Read the [Support](https://github.com/Forever-Uselesss/Roll/tree/main/support) documentation available in this repository.

## Repository Structure

| Path       | Description                        |
| ---------- | ---------------------------------- |
| `src/`     | Firmware source                    |
| `tests/`   | Tests                              |
| `support/` | Project documentation and support  |
| `.cargo/`  | Cargo and target configuration     |
| `.github/` | CI workflows                       |
