// https://adventofcode.com/2024/day/4
use std::time::Instant;

use gridthings::Grid;

const INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn solve() -> i32 {
    // Parse the input into a grid
    let grid: Grid<char> = Grid::from_str(INPUT).expect("Failed to parse grid");

    let mut xmas_count = 0;

    // For each cell in the grid
    for cell in grid.iter_cells() {
        // Get all possible 4-character segments starting from this cell
        let segments = grid.all_segments(cell.y, cell.x, 4);

        // Check each segment for "XMAS"
        for segment in segments {
            if segment.values == vec!['X', 'M', 'A', 'S'] {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

fn main() {
    let start_time = Instant::now();

    let answer = solve();
    println!("Found {} instances of XMAS", answer);
    println!("Time taken: {:?}", start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 18);
    }
}
