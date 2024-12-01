use aoc_2024::{day01, day02};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| day01::part1("01.txt")));
    c.bench_function("day01 part2", |b| b.iter(|| day01::part2("01.txt")));
    c.bench_function("day02 part1", |b| b.iter(|| day02::part1("02.txt")));
    c.bench_function("day02 part2", |b| b.iter(|| day02::part2("02.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
