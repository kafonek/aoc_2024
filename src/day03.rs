use regex::{Captures, Match, Regex};

pub fn part1(fname: &str) -> i32 {
    let content = std::fs::read_to_string(fname).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(&content) {
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        total += x * y;
    }

    total
}

enum Operation<'a> {
    Mul(Captures<'a>),
    Do(Match<'a>),
    Dont(Match<'a>),
}

impl<'a> Operation<'a> {
    fn start(&self) -> usize {
        match self {
            Operation::Mul(cap) => cap.get(0).unwrap().start(),
            Operation::Do(m) => m.start(),
            Operation::Dont(m) => m.start(),
        }
    }
}

pub fn part2(fname: &str) -> i32 {
    let content = std::fs::read_to_string(fname).unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut operations = Vec::new();
    for cap in mul_re.captures_iter(&content) {
        operations.push(Operation::Mul(cap));
    }
    for m in do_re.find_iter(&content) {
        operations.push(Operation::Do(m));
    }
    for m in dont_re.find_iter(&content) {
        operations.push(Operation::Dont(m));
    }

    operations.sort_by_key(|r| r.start());
    let mut total = 0;
    let mut enabled = true;
    for op in operations {
        match op {
            Operation::Mul(cap) => {
                if enabled {
                    let x = cap[1].parse::<i32>().unwrap();
                    let y = cap[2].parse::<i32>().unwrap();
                    total += x * y;
                }
            }
            Operation::Do(_m) => enabled = true,
            Operation::Dont(_m) => enabled = false,
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/03_01_sample.txt"), 161);
        assert_eq!(part1("data/03.txt"), 183669043);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/03_02_sample.txt"), 48);
        assert_eq!(part2("data/03.txt"), 59097164);
    }
}
