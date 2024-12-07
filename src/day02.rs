use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn read(fname: &str) -> Result<Vec<Vec<i32>>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut sequences = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        sequences.push(nums);
    }

    Ok(sequences)
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = (report[1] - report[0]).is_positive();
    debug!("({}) {:?}", if increasing { "inc" } else { "dec" }, report);
    let mut curr = report[0];
    for (i, &next) in report.iter().skip(1).enumerate() {
        let diff = curr - next;
        if (increasing && diff.is_positive()) || (!increasing && diff.is_negative()) {
            debug!("Direction change at index {} ({}, {})", i, curr, next);
            return false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            debug!("Diff out of range at index {} ({}, {})", i, curr, next);
            return false;
        }
        curr = next;
    }
    true
}

pub fn part1(fname: &str) -> i32 {
    let reports = read(fname).unwrap();
    let mut safe_count = 0;
    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn part2(fname: &str) -> i32 {
    let reports = read(fname).unwrap();
    let mut safe_count = 0;
    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
            continue;
        }
        // try dropping out each index at a time to see if it's safe
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_safe(&new_report) {
                safe_count += 1;
                break;
            }
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/02_sample.txt"), 2);
        assert_eq!(part1("data/02.txt"), 282);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/02_sample.txt"), 4);
        assert_eq!(part2("data/02.txt"), 349);
    }
}
