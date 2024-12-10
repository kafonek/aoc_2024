use gridthings::{Direction, Grid};
use rayon::prelude::*;
use std::collections::HashSet;

fn read(fname: &str) -> Grid<u32> {
    let content = std::fs::read_to_string(fname).unwrap();
    let data: Vec<Vec<u32>> = content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from_vecs(&data)
}

fn dfs_unique_peaks(
    grid: &Grid<u32>,
    y: i32,
    x: i32,
    visited: &mut HashSet<(i32, i32)>,
    peaks: &mut HashSet<(i32, i32)>,
) {
    let current_height = grid.get_value(y, x).unwrap();

    // If we reached a peak, record it
    if current_height == 9 {
        peaks.insert((y, x));
        return;
    }

    visited.insert((y, x));

    // Try each cardinal direction
    for neighbor in grid.get_cell_neighbors(y, x, Direction::cardinal()) {
        // Skip if already visited
        if visited.contains(&(neighbor.y, neighbor.x)) {
            continue;
        }

        // Only follow path if height increases by exactly 1
        if neighbor.value == current_height + 1 {
            dfs_unique_peaks(grid, neighbor.y, neighbor.x, visited, peaks);
        }
    }

    visited.remove(&(y, x));
}

fn dfs_count_paths(grid: &Grid<u32>, y: i32, x: i32, visited: &mut HashSet<(i32, i32)>) -> i32 {
    let current_height = grid.get_value(y, x).unwrap();

    // If we reached a peak, we found one complete path
    if current_height == 9 {
        return 1;
    }

    visited.insert((y, x));

    let mut path_count = 0;
    // Try each cardinal direction
    for neighbor in grid.get_cell_neighbors(y, x, Direction::cardinal()) {
        // Skip if already visited
        if visited.contains(&(neighbor.y, neighbor.x)) {
            continue;
        }

        // Only follow path if height increases by exactly 1
        if neighbor.value == current_height + 1 {
            path_count += dfs_count_paths(grid, neighbor.y, neighbor.x, visited);
        }
    }

    visited.remove(&(y, x));
    path_count
}

fn find_reachable_peaks(grid: &Grid<u32>, start_y: i32, start_x: i32) -> HashSet<(i32, i32)> {
    let mut peaks = HashSet::new();
    let mut visited = HashSet::new();
    dfs_unique_peaks(grid, start_y, start_x, &mut visited, &mut peaks);
    peaks
}

fn count_unique_paths(grid: &Grid<u32>, start_y: i32, start_x: i32) -> i32 {
    let mut visited = HashSet::new();
    dfs_count_paths(grid, start_y, start_x, &mut visited)
}

pub fn part1(fname: &str) -> i32 {
    let grid = read(fname);

    grid.iter_cells()
        .filter(|cell| cell.value == 0)
        .par_bridge()
        .map(|cell| {
            let peaks = find_reachable_peaks(&grid, cell.y, cell.x);
            peaks.len() as i32
        })
        .sum()
}

pub fn part2(fname: &str) -> i32 {
    let grid = read(fname);

    grid.iter_cells()
        .filter(|cell| cell.value == 0)
        .par_bridge()
        .map(|cell| count_unique_paths(&grid, cell.y, cell.x))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/10_sample.txt"), 36);
        assert_eq!(part1("data/10.txt"), 482);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/10_sample.txt"), 81);
        assert_eq!(part2("data/10.txt"), 1094);
    }
}
