# Advent of Code 2024 - Rust + WASM / Python bindings

## Rust Development

This year, I'll be trying to prototype solutions in Rust instead of in Python / Notebooks. `main.rs` is git-ignored but I use that as a scratchpad and can `cargo run` from the workspace level to execute that code. The configs in `.vscode/launch.json` enable setting breakpoints in `main.rs` or other files in `rust/src` and running in VSCode debug mode.

## Regression Tests

The `rust/`, `python/` and `pyo3/` repos have tests asserting that the solutions are correct, after manually solving them and verifying in the webpage. That enables refactoring with confdience.

 - `rust/`: `cargo test` from the workspace or rust directory
 - `python/`: `uv run pytest` from the python directory
 - `pyo3/`: `uv run pytest` from the pyo3 directory

## Benchmarks

I am using the `criterion` crate to benchmark Rust solutions, and `pytest-benchmark` for the pure Python and PyO3 solutions.

 - `rust/`: `cargo bench` from the workspace or rust directory
 - `python/`: `uv run pytest --benchmark-only` from the python directory
 - `pyo3/`: `uv run pytest --benchmark-only` from the pyo3 directory

## WASM

- `cargo install wasm-pack`
- `cd wasm`
- `wasm-pack build --target web --out-dir static/pkg`
- `uv run python -m http.server --directory static`