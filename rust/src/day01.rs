use std::collections::HashMap;

use crate::utils::FromLines;

#[derive(Debug)]
pub struct Day01Part1 {
    // both Vectors sorted smallest to largest and should be same length
    left: Vec<i32>,
    right: Vec<i32>,
}

impl FromLines for Day01Part1 {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in lines {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok());

            if let (Some(l), Some(r)) = (nums.next(), nums.next()) {
                left.push(l);
                right.push(r);
            }
        }

        Day01Part1 { left, right }
    }
}

impl Day01Part1 {
    pub fn calculate_distance(&self) -> i32 {
        let mut left_sorted = self.left.clone();
        let mut right_sorted = self.right.clone();

        left_sorted.sort_unstable();
        right_sorted.sort_unstable();

        let mut total_distance = 0;
        for (l, r) in left_sorted.iter().zip(right_sorted.iter()) {
            let distance = (l - r).abs();
            total_distance += distance;
        }

        total_distance
    }

    pub fn similarity(&self) -> i32 {
        let mut right_side_counts: HashMap<i32, i32> = HashMap::new();
        let mut left_side_totals: HashMap<i32, i32> = HashMap::new();

        for r in self.right.iter() {
            *right_side_counts.entry(*r).or_insert(0) += 1;
        }

        for l in self.left.iter() {
            let val = l * right_side_counts.get(l).unwrap_or(&0);
            *left_side_totals.entry(*l).or_insert(0) += val;
        }

        left_side_totals.values().sum()
    }
}
