use aoc_2024::day01;

#[test]
fn test_day01() {
    assert_eq!(day01::part1("data/01_sample.txt"), 11);
    assert_eq!(day01::part1("data/01.txt"), 2344935);
    assert_eq!(day01::part2("data/01_sample.txt"), 31);
    assert_eq!(day01::part2("data/01.txt"), 27647262);
}
