// https://adventofcode.com/2023/day/3
use std::time::Instant;

use gridthings::{Cell, Direction, Grid};

const INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

#[derive(Debug)]
struct ContiguousNumber {
    value: u32,
    cells: Vec<Cell<char>>,
}

fn solve() -> u32 {
    let data: Vec<Vec<char>> = INPUT
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let grid = Grid::from_vecs(&data);
    println!(
        "Parsed grid with {} rows and {} columns",
        grid.row_len, grid.col_len
    );

    // First, identify continguous numbers
    let mut collections = Vec::new();
    for row in grid.iter_rows() {
        let mut curr_cells = Vec::new();
        for cell in row {
            if cell.value.is_digit(10) {
                curr_cells.push(cell);
            } else {
                if curr_cells.len() > 1 {
                    collections.push(curr_cells);
                    curr_cells = Vec::new();
                }
            }
        }
    }
    let mut continguous_numbers = Vec::new();
    for collection in collections {
        let value: u32 = collection
            .iter()
            .map(|c| c.value)
            .collect::<String>()
            .parse()
            .unwrap();
        continguous_numbers.push(ContiguousNumber {
            value,
            cells: collection,
        });
    }

    println!(
        "Identified {} continguous numbers. First one: {:?}",
        continguous_numbers.len(),
        continguous_numbers[0]
    );

    // Next, find the ones that are touching any special character (not a . or number)
    let mut special_char_adjacent = Vec::new();
    let mut not_adjacent = Vec::new();
    for continguous_number in continguous_numbers {
        let mut is_adjacent = false;
        for cell in continguous_number.cells {
            for neighbor in grid.get_cell_neighbors(cell.row, cell.col, Direction::all()) {
                if !neighbor.value.is_digit(10) && neighbor.value != '.' {
                    is_adjacent = true;
                }
            }
        }
        if is_adjacent {
            special_char_adjacent.push(continguous_number.value);
        } else {
            not_adjacent.push(continguous_number.value);
        }
    }

    println!("Special character adjacent: {:?}", special_char_adjacent);
    println!("Not adjacent: {:?}", not_adjacent);

    special_char_adjacent.iter().sum::<u32>()
}

fn main() {
    let start_time = Instant::now();

    let answer = solve();
    println!("Answer: {}", answer);
    println!("Time taken: {:?}", start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 4361);
    }
}
