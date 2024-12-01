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
}
