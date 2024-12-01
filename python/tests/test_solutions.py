from aoc_2024 import day01


def test_day01():
    assert day01.part1("01_sample.txt") == 11
    assert day01.part1("01.txt") == 2344935
    assert day01.part2("01_sample.txt") == 31
    assert day01.part2("01.txt") == 27647262


def test_01_01_benchmark(benchmark):
    benchmark(day01.part1, "01.txt")


def test_01_02_benchmark(benchmark):
    benchmark(day01.part2, "01.txt")
