use aoc_2024::{day01, day02, day03, day04, day05, day06, day07, day08};

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| day01::part1("data/01.txt")));
    c.bench_function("day01 part2", |b| b.iter(|| day01::part2("data/01.txt")));
    c.bench_function("day02 part1", |b| b.iter(|| day02::part1("data/02.txt")));
    c.bench_function("day02 part2", |b| b.iter(|| day02::part2("data/02.txt")));
    c.bench_function("day03 part1", |b| b.iter(|| day03::part1("data/03.txt")));
    c.bench_function("day03 part2", |b| b.iter(|| day03::part2("data/03.txt")));
    c.bench_function("day04 part1", |b| b.iter(|| day04::part1("data/04.txt")));
    c.bench_function("day04 part2", |b| b.iter(|| day04::part2("data/04.txt")));
    c.bench_function("day05 part1", |b| b.iter(|| day05::part1("data/05.txt")));
    c.bench_function("day05 part2", |b| b.iter(|| day05::part2("data/05.txt")));
    c.bench_function("day06 part1", |b| b.iter(|| day06::part1("data/06.txt")));
    c.bench_function("day06 part2", |b| b.iter(|| day06::part2("data/06.txt")));
    c.bench_function("day07 part1", |b| b.iter(|| day07::part1("data/07.txt")));
    c.bench_function("day07 part2", |b| b.iter(|| day07::part2("data/07.txt")));
    c.bench_function("day08 part1", |b| b.iter(|| day08::part1("data/08.txt")));
    c.bench_function("day08 part2", |b| b.iter(|| day08::part2("data/08.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
