# Advent of Code 2024 - Rust + WASM / Python bindings

## Execute Binary

`cargo run --bin <day>_<part>`, e.g. `cargo run --bin 01_01`

## Breakpoint Debugging

This year, I'll be trying to prototype solutions in Rust instead of in Python / Notebooks. 

Breakpoint debugging: see `.vscode/launch.json` for configs. With the `rust-analyzer` and `CodeLLDB` extensions, I can set breakpoints in any of the `rust/` files, select a binary in `rust/src/bin/` and use `Run -> Start Debugging` to launch the program with breakpoints.

