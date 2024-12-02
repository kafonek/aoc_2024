from aoc_2024 import day01, day02


def test_day01():
    assert day01.part1("01_sample.txt") == 11
    assert day01.part1("01.txt") == 2344935
    assert day01.part2("01_sample.txt") == 31
    assert day01.part2("01.txt") == 27647262


def test_01_01_benchmark(benchmark):
    benchmark(day01.part1, "01.txt")


def test_01_02_benchmark(benchmark):
    benchmark(day01.part2, "01.txt")


def test_day02():
    assert day02.part1("02_sample.txt") == 2
    assert day02.part1("02.txt") == 282
    assert day02.part2("02_sample.txt") == 4
    assert day02.part2("02.txt") == 349


def test_02_01_benchmark(benchmark):
    benchmark(day02.part1, "02.txt")


def test_02_02_benchmark(benchmark):
    benchmark(day02.part2, "02.txt")
