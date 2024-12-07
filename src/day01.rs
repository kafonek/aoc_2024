use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn read(fname: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let (l, r) = line.split_once(" ").unwrap();
        left.push(l.trim().parse::<i32>().unwrap());
        right.push(r.trim().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    Ok((left, right))
}

#[allow(dead_code)]
pub fn part1(fname: &str) -> i32 {
    let (left, right) = read(fname).unwrap();

    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += (l - r).abs();
    }
    total_distance
}

#[allow(dead_code)]
pub fn part2(fname: &str) -> i32 {
    let (left, right) = read(fname).unwrap();

    // Create frequency map for right side
    let mut right_counts = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &num in &left {
        if let Some(&count) = right_counts.get(&num) {
            similarity_score += num * count;
        }
    }
    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/01_sample.txt"), 11);
        assert_eq!(part1("data/01.txt"), 2344935);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/01_sample.txt"), 31);
        assert_eq!(part2("data/01.txt"), 27647262);
    }
}
