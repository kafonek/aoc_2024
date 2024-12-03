use crate::utils::{read_text, FromText};
use regex::Regex;

pub fn part1(filename: &str) -> i32 {
    let data: Day03 = read_text(filename).unwrap();
    data.sum_multiplications()
}

pub fn part2(filename: &str) -> i32 {
    let data: Day03 = read_text(filename).unwrap();
    data.sum_multiplications_with_conditionals()
}

#[derive(Debug)]
pub struct Day03 {
    content: String,
}

impl FromText for Day03 {
    fn from_text(text: &str) -> Self {
        Day03 {
            content: text.to_string(),
        }
    }
}

impl Day03 {
    fn sum_multiplications(&self) -> i32 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        re.captures_iter(&self.content)
            .filter_map(|cap| {
                let x = cap[1].parse::<i32>().ok()?;
                let y = cap[2].parse::<i32>().ok()?;
                Some(x * y)
            })
            .sum()
    }

    fn sum_multiplications_with_conditionals(&self) -> i32 {
        let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        // Get all matches with their positions
        let mut operations: Vec<(usize, &str)> = Vec::new();

        // Collect mul operations with their positions
        for cap in mul_re.captures_iter(&self.content) {
            let pos = cap.get(0).unwrap().start();
            operations.push((pos, "mul"));
        }

        // Collect do/don't operations with their positions
        for m in do_re.find_iter(&self.content) {
            operations.push((m.start(), "do"));
        }
        for m in dont_re.find_iter(&self.content) {
            operations.push((m.start(), "dont"));
        }

        // Sort by position to process in order
        operations.sort_by_key(|&(pos, _)| pos);

        let mut enabled = true;
        let mut total = 0;

        for (pos, op) in operations {
            match op {
                "mul" => {
                    if enabled {
                        // Get the numbers from the multiplication at this position
                        if let Some(cap) = mul_re.captures(&self.content[pos..]) {
                            let x = cap[1].parse::<i32>().unwrap();
                            let y = cap[2].parse::<i32>().unwrap();
                            total += x * y;
                        }
                    }
                }
                "do" => enabled = true,
                "dont" => enabled = false,
                _ => unreachable!(),
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Day03 {
            content: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                .to_string(),
        };
        assert_eq!(input.sum_multiplications(), 161);
    }

    #[test]
    fn test_example_part2() {
        let input = Day03 {
            content: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))"
                .to_string(),
        };
        assert_eq!(input.sum_multiplications_with_conditionals(), 48);
    }
}
