# Kice OS

## Overview
Kice OS is a custom operating system kernel project.

## Development Setup
1. Ensure you have Rust installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install required tools: `rustup component add rust-src`
3. Build the project: `cargo build`

## Project Structure
- `src/`: Source code directory
  - `lib.rs`: Main library entry point
  - `asm/`: Assembly language files
  - `kernel/`: Kernel-specific implementations
  - `drivers/`: Device driver implementations
  - `utils/`: Utility functions and helpers

## Building
- Development build: `cargo build`
- Release build: `cargo build --release`

## Contributing
Please read CONTRIBUTING.md for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
