use aoc_2024::day01;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| day01::part1("01.txt")));
    c.bench_function("day01 part2", |b| b.iter(|| day01::part2("01.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
