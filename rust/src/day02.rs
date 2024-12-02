use crate::utils::{read_lines, FromLines};

pub fn part1(filename: &str) -> i32 {
    let data: Day02 = read_lines(filename).unwrap();
    data.count_safe_reports()
}

pub fn part2(filename: &str) -> i32 {
    let data: Day02 = read_lines(filename).unwrap();
    data.count_safe_reports_with_dampener()
}

#[derive(Debug)]
pub struct Day02 {
    reports: Vec<Vec<i32>>,
}

impl FromLines for Day02 {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let reports = lines
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect()
            })
            .collect();

        Day02 { reports }
    }
}

impl Day02 {
    fn is_safe_report(report: &[i32]) -> bool {
        if report.len() < 2 {
            return false;
        }

        // Check first pair to determine if we should be increasing or decreasing
        let increasing = report[1] > report[0];
        let mut prev = report[0];

        for &curr in &report[1..] {
            let diff = curr - prev;

            // Check rules:
            // 1. Must maintain increasing/decreasing pattern
            // 2. Difference must be between 1 and 3 (inclusive)
            if increasing && (diff <= 0 || diff > 3) {
                return false;
            }
            if !increasing && (diff >= 0 || diff < -3) {
                return false;
            }

            prev = curr;
        }

        true
    }

    pub fn count_safe_reports(&self) -> i32 {
        self.reports
            .iter()
            .filter(|report| Self::is_safe_report(report))
            .count() as i32
    }

    pub fn count_safe_reports_with_dampener(&self) -> i32 {
        self.reports
            .iter()
            .filter(|report| {
                // First check if it's safe without removing anything
                if Self::is_safe_report(report) {
                    return true;
                }

                // Try removing each number one at a time
                for skip_idx in 0..report.len() {
                    let filtered: Vec<i32> = report
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip_idx)
                        .map(|(_, &x)| x)
                        .collect();

                    if Self::is_safe_report(&filtered) {
                        return true;
                    }
                }

                false
            })
            .count() as i32
    }
}
