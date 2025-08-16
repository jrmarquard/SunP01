# Gemini Rust Project Guide

This document provides a general guide for working on this Rust project.

## 1. Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/lib.rs`: If this were a library, this would be the main entry point.
- `Cargo.toml`: The project's manifest file, containing metadata and dependencies.
- `target/`: This directory contains the build artifacts.

## 2. Dependencies

Dependencies are managed in `Cargo.toml`. To add a new dependency, you can either manually add it to the `[dependencies]` section or use the `cargo add` command:

```bash
cargo add <dependency-name>
```

## 3. Building the Project

To build the project, use the following command:

```bash
cargo build
```

For a release build, use the `--release` flag:

```bash
cargo build --release
```

## 4. Running the Project

To run the project, use the following command:

```bash
cargo run
```

For a release build, use the `--release` flag:

```bash
cargo run --release
```

## 5. Testing

To run the tests, use the following command:

```bash
cargo test
```

## 6. Linting

To lint the project, use `clippy`:

```bash
cargo clippy
```

This project is also configured to run `clippy` in a GitHub Action on every push.

## 7. Formatting

To format the code, use `rustfmt`:

```bash
cargo fmt
```

## 8. VSCode Workspace

The `.vscode/settings.json` file contains recommended settings for this project, including enabling `rust-analyzer` and formatting on save.

## 9. Communication

Prioritise readability for a new developer. Use clear and descriptive variable names, comments, and documentation. Do not go overboard and keep communication concise.