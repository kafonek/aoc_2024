use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read(fname: &str) -> Vec<Equation> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| Equation::parse(line.unwrap()))
        .collect()
}

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn parse(line: String) -> Self {
        let (target, numbers) = line.split_once(":").unwrap();
        let target = target.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Equation { target, numbers }
    }

    fn is_valid(&self, ops: &[Operator]) -> bool {
        let mut total = self.numbers[0];
        for (i, op) in ops.iter().enumerate() {
            total = op.eval(total, self.numbers[i + 1]);
        }
        debug!("{:?} {:?}, {}", self, ops, total);
        total == self.target
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn eval(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concatenate => {
                let right_str = right.to_string();
                left * 10_u64.pow(right_str.len() as u32) + right
            }
        }
    }

    fn part1_cycle(length: usize) -> impl Iterator<Item = Vec<Operator>> {
        std::iter::repeat([Operator::Add, Operator::Multiply])
            .take(length)
            .multi_cartesian_product()
    }

    fn part2_cycle(length: usize) -> impl Iterator<Item = Vec<Operator>> {
        std::iter::repeat([Operator::Add, Operator::Multiply, Operator::Concatenate])
            .take(length)
            .multi_cartesian_product()
    }
}

pub fn part1(fname: &str) -> u64 {
    let equations = read(fname);
    equations
        .par_iter()
        .map(|eq| {
            let ops_cycle = Operator::part1_cycle(eq.numbers.len() - 1);
            for ops in ops_cycle {
                if eq.is_valid(&ops) {
                    return eq.target;
                }
            }
            0
        })
        .sum()
}

pub fn part2(fname: &str) -> u64 {
    let equations = read(fname);
    equations
        .par_iter()
        .map(|eq| {
            let ops_cycle = Operator::part2_cycle(eq.numbers.len() - 1);
            for ops in ops_cycle {
                if eq.is_valid(&ops) {
                    return eq.target;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/07_sample.txt"), 3749);
        assert_eq!(part1("data/07.txt"), 3245122495150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/07_sample.txt"), 11387);
        assert_eq!(part2("data/07.txt"), 105517128211543);
    }
}
