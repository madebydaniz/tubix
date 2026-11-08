# Tubix

A cross-platform terminal interface for managing YouTube downloads, written in Rust.

Tubix is in early development. The current build provides a dark dashboard,
keyboard help, and safe terminal startup and shutdown. Downloading is not
available yet.

## Install from source

Install [stable Rust](https://rust-lang.org/tools/install/), then:

```sh
git clone https://github.com/madebydaniz/tubix.git
cd tubix
git switch develop
cargo install --path . --locked
```

The foundation requires Rust 1.99 or newer. Precompiled releases, crates.io
installation, and Homebrew installation are planned and are not available yet.

## Usage

```sh
tubix
tubix --help
tubix --version
```

The dashboard requires an interactive terminal. It does not access YouTube or
install external tools.

| Key | Action |
| --- | --- |
| `q` / `Ctrl+C` | Quit, including while help is open |
| `?` | Toggle keyboard help |
| `Esc` | Close help |

Optional diagnostics are written to the file specified by `TUBIX_LOG`.
Logs are disabled by default and never printed over the dashboard.

## Contributing

Create a `feat/*` branch from `develop` and open a pull request to `develop`.
Use Conventional Commits with a single subject of at most 70 characters.
Keep all project tests under `tests/` and run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

See [architecture](docs/ARCHITECTURE.md), [decisions](docs/DECISIONS.md),
and [roadmap](docs/ROADMAP.md) for scope and development boundaries.

[Ask a question](https://github.com/madebydaniz/tubix/discussions)
or [support development](https://github.com/sponsors/madebydaniz).

Licensed under the [MIT License](LICENSE).
