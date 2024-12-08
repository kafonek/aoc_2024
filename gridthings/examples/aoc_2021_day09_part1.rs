// https://adventofcode.com/2021/day/9

use gridthings::{Direction, Grid};
use std::time::Instant;

const INPUT: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

fn solve() -> u32 {
    // Parse the input into a grid of u8 digits
    let data: Vec<Vec<u32>> = INPUT
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let grid = Grid::from_vecs(&data);
    println!(
        "Parsed grid with {} rows and {} columns",
        grid.row_len, grid.col_len
    );

    let mut risk_level_sum = 0;

    // For each cell in the grid
    for cell in grid.iter_cells() {
        let mut is_low_point = true;

        // Check all cardinal neighbors (up, right, down, left)
        for neighbor in grid.get_cell_neighbors(cell.row, cell.col, Direction::cardinal()) {
            if neighbor.value <= cell.value {
                is_low_point = false;
                break;
            }
        }

        if is_low_point {
            // Risk level is 1 plus the height
            risk_level_sum += 1 + cell.value;
        }
    }

    risk_level_sum
}

fn main() {
    let start_time = Instant::now();
    let answer = solve();
    println!("Sum of risk levels: {}", answer);
    println!("Time taken: {:?}", start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 15);
    }
}
