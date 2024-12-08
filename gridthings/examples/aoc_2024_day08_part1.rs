// https://adventofcode.com/2024/day/8
// - Find cells with matching values (A, 0)
// - Calculate "antinode" locations for all combo of cells with same value
// - "antinode" can be found by finding the x, y distance between cells and
//   then moving that distance in both directions from each cell
//   e.g. (1, 1), (2, 2) -> (3, 3), (0, 0)
// - Note antinodes from different antennae can overlap and need to count both
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use gridthings::Grid;

const INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

// For visual ref only
const _OUTPUT: &str = r#"
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
"#;

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

    let mut node_groups = HashMap::new();
    for cell in grid.iter_cells() {
        if cell.value != '.' {
            node_groups
                .entry(cell.value)
                .or_insert_with(Vec::new)
                .push(cell);
        }
    }
    println!("Node groups: {:?}", node_groups);

    let mut antinodes = HashSet::new();

    // For each group of same-value nodes
    for (value, cells) in node_groups.iter() {
        // For each pair of cells with same frequency
        for (i, cell1) in cells.iter().enumerate() {
            for cell2 in cells.iter().skip(i + 1) {
                // Calculate vector between cells
                let y_diff = cell2.y as i32 - cell1.y as i32;
                let x_diff = cell2.x as i32 - cell1.x as i32;

                // Calculate antinode positions (one in each direction from the cells)
                let antinode1_y = cell1.y - y_diff;
                let antinode1_x = cell1.x - x_diff;
                let antinode2_y = cell2.y + y_diff;
                let antinode2_x = cell2.x + x_diff;

                // Add antinodes if they're in bounds and not occupied
                for &(new_y, new_x) in &[(antinode1_y, antinode1_x), (antinode2_y, antinode2_x)] {
                    if new_y >= 0
                        && new_x >= 0
                        && new_y < grid.col_len as i32
                        && new_x < grid.row_len as i32
                        && !node_groups
                            .values()
                            .any(|cells| cells.iter().any(|c| c.y == new_y && c.x == new_x))
                    {
                        antinodes.insert((new_y, new_x, *value));
                    }
                }
            }
        }
    }
    let mut sorted_antinodes = antinodes.iter().collect::<Vec<_>>();
    sorted_antinodes.sort();
    println!("Antinodes: {:?}", sorted_antinodes);
    sorted_antinodes.len() as u32
}

fn main() {
    let start_time = Instant::now();

    let answer = solve();
    println!("Total antinodes: {}", answer);
    println!("Time taken: {:?}", start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 14);
    }
}
