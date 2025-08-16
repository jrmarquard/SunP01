# SunP01 Server

This is a basic Rust HTTP server.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Change into the project directory:

```bash
cd SunP01
```

## Developer Setup

1. Build the project:

```bash
cargo build
```

2. Run the server:

```bash
cargo run
```

The server will be running at `http://127.0.0.1:8080`.

## Packaging and Running the Server

1. Build the project for release:

```bash
cargo build --release
```

2. Run the release build:

```bash
./target/release/sunp01
```

The server will be running at `http://127.0.0.1:8080`.