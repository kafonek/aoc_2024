# Advent of Code 2024 - Rust + WASM / Python bindings

## Rust Development

This year, I'll be trying to prototype solutions in Rust instead of in Python / Notebooks. `main.rs` is git-ignored but I use that as a scratchpad and can `cargo run` from the workspace level to execute that code. The configs in `.vscode/launch.json` enable setting breakpoints in `main.rs` or other files in `rust/src` and running in VSCode debug mode.

## Regression Tests

One goal this year is to embrace Rust's `fearless refactoring` paradigm. To that end, once code is working I'll add a regression test to `tests/solutions.rs`. Those can be run with `cargo test` from the workspace level.