# Contributing

## Before You Start

Please familiarize yourself with the following:
>If you are new to the project, the [Quick Start Guide](https://github.com/Forever-Uselesss/Roll/blob/main/support/documentation/quick_start.md) is the best place to begin.

- [README](https://github.com/Forever-Uselesss/Roll/tree/main/README.md)
- [CODE OF CONDUCT](https://github.com/Forever-Uselesss/Roll/tree/main/CODE_OF_CONDUCT.md)
- [Support Documents Available](https://github.com/Forever-Uselesss/Roll/tree/main/support)
  - [Quick Start Guide](https://github.com/Forever-Uselesss/Roll/tree/main/support/documentation/quick_start.md)

## Development Setup
>Please refer to [Quick Start Guide](https://github.com/Forever-Uselesss/Roll/tree/main/support/documentation/quick_start.md) for initial setup and get the project running.

## Making Changes

1. Create a branch
    ```bash
    git switch -c <your-branch-name>
    ``` 

2. Make focused changes and avoid unrelated modifications
3. Follow the existing code documentation style (`cargo fmt`) and have plenty of comments
4. Run the recommended checks

    ```bash
    # Check formatting
    cargo fmt --all -- --check

    # Run Clippy
    cargo clippy --verbose --all-features --workspace -- -D warnings
        
    # Run Audit
    cargo audit

    # Build the project
    cargo build

    # Run tests
    cargo test
    ```
    
5. Build for the target (ESP32-C6)

    ```bash
    cargo build --verbose --profile dev --all-features
    ```
  
6. Test on hardware when applicable
7. Open a pull request

## Commit Messages

Use concise imperative messages:

Good examples:

  * Add button input driver
  * Fix dice range calculation
  * Update quick start instructions

Avoid vague messages such as:

  * Fix stuff
  * Changes
  * Update
  * It works now


## Pull Requests

Please describe:

- What changed
- Why it changed
- How it was tested
- Hardware used for testing
- Any known limitations or follow-up work

When applicable, include screenshots, serial output, test results, or other information that helps reviewers understand and verify the change.


## Accessibility

Accessibility is a core part of DICE. Contributions that improve usability or accommodate different physical abilities are especially valuable.

When making accessibility-related changes, consider:

  * Different methods of physical interaction
  * Customization of controls and behavior
  * Clear and understandable feedback
  * Reducing unnecessary physical requirements
  * Whether a change introduces new physical barriers

Please avoid assuming that one interaction method will work for every user.


## Questions and Support

If you are unsure how to proceed, consult the [Support Documentation](#before-you-start) first.

If your question is not addressed there, open an issue with enough information for others to reproduce or understand the problem.
