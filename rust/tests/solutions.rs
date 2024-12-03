use aoc_2024::day01;
use aoc_2024::day02;
use aoc_2024::day03;

#[test]
fn test_day01() {
    assert_eq!(day01::part1("01_sample.txt"), 11);
    assert_eq!(day01::part1("01.txt"), 2344935);
    assert_eq!(day01::part2("01_sample.txt"), 31);
    assert_eq!(day01::part2("01.txt"), 27647262);
}

#[test]
fn test_day02() {
    assert_eq!(day02::part1("02_sample.txt"), 2);
    assert_eq!(day02::part1("02.txt"), 282);
    assert_eq!(day02::part2("02_sample.txt"), 4);
    assert_eq!(day02::part2("02.txt"), 349);
}

#[test]
fn test_day03() {
    assert_eq!(day03::part1("03_01_sample.txt"), 161);
    assert_eq!(day03::part1("03.txt"), 183669043);
    assert_eq!(day03::part2("03_02_sample.txt"), 48);
    assert_eq!(day03::part2("03.txt"), 59097164);
}
