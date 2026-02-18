# Reckon Bolt

Valorant scrims retrieval desktop app built with Rust and Tauri v2.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Tauri CLI](https://tauri.app/start/): `cargo install tauri-cli --version "^2"`
- Windows: WebView2 (pre-installed on Windows 10/11)

## Development

```bash
# Run in development mode (from project root)
cargo tauri dev
```

## Build

```bash
# Create a production build
cargo tauri build
```

## Project Structure

```
reckon_bolt_app/
├── src/                  # Frontend (HTML/CSS/JS)
│   ├── index.html
│   └── styles.css
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   └── lib.rs        # Tauri app setup & commands
│   ├── Cargo.toml
│   ├── build.rs
│   └── tauri.conf.json   # Tauri configuration
└── README.md
```
