from aoc_2024_pyo3 import day01


def test_day01():
    assert day01.part1("01_sample.txt") == 11
    assert day01.part1("01.txt") == 2344935
    assert day01.part2("01_sample.txt") == 31
    assert day01.part2("01.txt") == 27647262
