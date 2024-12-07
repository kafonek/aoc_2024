# Advent of Code 2024

For AOC 2023, I explored creating WASM and pyo3 bindings for Rust and benchmarking the performance compared to pure Python solutions. That took a lot of time. This year, I'm focusing on the developer experience of solving only in Rust. While I initially did create pyo3 and wasm bindings, the compile times for pyo3 in particular were a pain point so I've recreated this repo without them.

## Developer workflow

1. Write sample and input data in `data/`
2. Create a `src/dayNN.rs` file
3. Edit `src/main.rs` to run the part you're working on
4. Once a solution is working, add unit tests in `src/dayNN.rs`

## Style Guide

 - Avoid hard to read multi-line functional calls
 - Avoid lifetimes and high-level abstractions in favor of simplicity, can use liberal `.clone()` if needed
 