# Rust Competitive Programming Setup

This repository contains a merged Rust competitive-programming template and reusable algorithms/data structures.

## Prerequisites

1. Rust toolchain (recommended: 1.85.1)
2. Python 3 (for `run.py` helper)

## Install Tooling

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup default 1.85.1

+cargo install rust_bundler_cp
```

## Project Setup

```bash
cargo fetch
```

This downloads crate dependencies listed in `Cargo.toml`.

## Build And Test

```bash
cargo build

cargo test
```

## Run Solutions (Cargo)

Binaries are defined in `src/bin/` (for example `a.rs`, `b.rs`, ..., `g.rs`, and `main.rs`).

```bash
cargo run --bin rust_codeforce_template

cargo run --bin a
cargo run --bin b
```

## Run Bundled Single-File Build (`run.py`)

The helper script can bundle one binary into a single source file (useful for submissions), compile it with `rustc`, and run it.

```bash
python3 run.py

python3 run.py a
python3 run.py b
```

Bundled output is written under `result/`.

### Reset Problem Files

If you used the template workflow and want to reset `src/bin/*.rs` (except `_template.rs`) while backing up current files:

```bash
python3 run.py --reset
```

Backups are stored in `backup/<timestamp>/`.
