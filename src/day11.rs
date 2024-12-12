use crate::utils::split_number;
use count_digits::CountDigits;
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_stones(fname: &str) -> Vec<u64> {
    let content = read_to_string(fname).unwrap();
    content
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct CacheKey {
    value: u64,
    remaining: u32,
}

fn blink_cached(value: u64, remaining: u32, cache: &mut HashMap<CacheKey, u64>) -> u64 {
    let key = CacheKey { value, remaining };

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = if remaining == 1 {
        if value == 0 {
            1
        } else if value.count_digits() % 2 == 0 {
            2
        } else {
            1
        }
    } else {
        if value == 0 {
            blink_cached(1, remaining - 1, cache)
        } else if value.count_digits() % 2 == 0 {
            let (left, right) = split_number(value);
            if left == right {
                2 * blink_cached(left, remaining - 1, cache)
            } else {
                blink_cached(left, remaining - 1, cache) + blink_cached(right, remaining - 1, cache)
            }
        } else {
            blink_cached(value * 2024, remaining - 1, cache)
        }
    };

    cache.insert(key, result);
    result
}

pub fn part1(fname: &str) -> u64 {
    let stones = read_stones(fname);
    let mut cache = HashMap::new();
    let n = 25;
    let final_count: u64 = stones
        .iter()
        .map(|&stone| blink_cached(stone, n, &mut cache))
        .sum();
    final_count
}

pub fn part2(fname: &str) -> u64 {
    let stones = read_stones(fname);
    let mut cache = HashMap::new();
    let n = 75;
    let final_count: u64 = stones
        .iter()
        .map(|&stone| blink_cached(stone, n, &mut cache))
        .sum();
    final_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("data/day11_sample.txt"), 55312);
        assert_eq!(part1("data/day11.txt"), 187738);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("data/day11.txt"), 223767210249237);
    }
}
