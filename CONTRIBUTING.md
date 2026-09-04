# Contributing

## Before You Start

Please familiarize yourself with the following:
>If you are new to the project, the [Quick Start Guide](/support/documentation/quick_start.md) is the best place to begin.

- [README](https://github.com/Forever-Uselesss/Roll/tree/main/README.md)
- [CODE OF CONDUCT](https://github.com/Forever-Uselesss/Roll/tree/main/CODE_OF_CONDUCT.md)
- [Support Documents Available](https://github.com/Forever-Uselesss/Roll/tree/main/support)
  - [Quick Start Guide](https://github.com/Forever-Uselesss/Roll/tree/main/support/documentation/quick_start.md)

## Development Setup
>Please refer to [Quick Start Guide](/support/documentation/quick_start.md) for initial setup and get the project running.

Different IDEs do things differently. Please install your preferred extensions beforehand.

## Development Flow

Keep your changes focused and atomic. Avoid combining unrelated bug fixes, feature additions, and documentation tweaks into a single commit.

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
    cargo build --profile dev --all-features

    ```
    
5. Build for the target (ESP32-C6)

    ```bash
    cargo build --verbose --profile dev --all-features
    ```
  
6. Test on hardware when applicable
7. Open a pull request

## Hardware Testing

If your change interacts with hardware (e.g., a new button or sensor), please include details on the hardware and the procedure you used to verify it. Testing on hardware is very easy with Cargo test. All you have to do is run  `cargo test --all-features --target <your-build-target>` with an MCU connected to your computer and cargo runs available regression tests.

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

When you are ready to submit, please create a Pull Request and ensure you include the following in your description:

1. Summary: A concise description of the change (the "what").
2. Motivation: Why this change was necessary (the "why").
3. Verification Steps: Detailed instructions on how to reproduce and verify the fix/feature.
4. Hardware Context: What specific physical hardware was used for testing.
5. Known Issues: Any known limitations or follow-up work items.

When applicable, include screenshots, serial output, test results, or other information that helps reviewers understand and verify the change.


## Accessibility

Accessibility is a core part of Roll. Contributions that improve usability or accommodate different physical abilities are especially valuable.

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
