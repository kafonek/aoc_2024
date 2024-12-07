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
 - Use logging not print statements while iterating on a solution

## Benchmarks

From `cargo criterion`:

```
day01 part1             time:   [130.40 µs 131.61 µs 132.56 µs]                        
day01 part2             time:   [149.91 µs 150.41 µs 151.00 µs]                        

day02 part1             time:   [210.89 µs 213.28 µs 215.61 µs]                        
day02 part2             time:   [340.30 µs 342.93 µs 346.07 µs]                        

day03 part1             time:   [373.74 µs 375.40 µs 377.33 µs]                        
day03 part2             time:   [422.36 µs 423.93 µs 425.82 µs]                        

day04 part1             time:   [8.4948 ms 8.7272 ms 8.9484 ms]                        
day04 part2             time:   [798.51 µs 813.75 µs 829.55 µs]                        

day05 part1             time:   [4.3618 ms 4.3757 ms 4.3919 ms]                         
day05 part2             time:   [6.4606 ms 6.4740 ms 6.4877 ms]                        

day06 part1             time:   [470.09 µs 472.87 µs 475.75 µs]                        
day06 part2             time:   [115.21 ms 116.44 ms 117.69 ms]                        

day07 part1             time:   [5.0460 ms 5.2199 ms 5.3922 ms]                         
day07 part2             time:   [50.467 ms 51.372 ms 52.261 ms]    
```