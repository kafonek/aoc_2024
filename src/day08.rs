use gridthings::{Cell, Grid};
use log::debug;
use std::collections::{HashMap, HashSet};

fn read(fname: &str) -> Grid<char> {
    let content = std::fs::read_to_string(fname).unwrap();
    let data: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    Grid::from_vecs(&data)
}

fn find_antinodes_part1(grid: &Grid<char>) -> HashSet<(usize, usize)> {
    // Group cells by frequency
    let mut freq_map: HashMap<char, Vec<Cell<char>>> = HashMap::new();
    for cell in grid.iter_cells() {
        if cell.value != '.' {
            freq_map.entry(cell.value).or_default().push(cell.clone());
        }
    }
    debug!("Freq map: {:?}", freq_map);

    let mut antinodes = HashSet::new();

    // For each frequency group
    for cells in freq_map.values() {
        // For each pair of cells with same frequency
        for (i, cell1) in cells.iter().enumerate() {
            for cell2 in cells.iter().skip(i + 1) {
                // Calculate vector between antennas
                let row_diff = cell2.row as i32 - cell1.row as i32;
                let col_diff = cell2.col as i32 - cell1.col as i32;

                // Calculate antinode positions
                let antinode1 = (
                    (cell1.row as i32 - row_diff) as usize,
                    (cell1.col as i32 - col_diff) as usize,
                );
                let antinode2 = (
                    (cell2.row as i32 + row_diff) as usize,
                    (cell2.col as i32 + col_diff) as usize,
                );

                // Only add antinodes that are within grid bounds
                if antinode1.0 < grid.row_len && antinode1.1 < grid.col_len {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 < grid.row_len && antinode2.1 < grid.col_len {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

fn find_points_on_line(
    p1: (usize, usize),
    p2: (usize, usize),
    grid: &Grid<char>,
) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let (x1, y1) = (p1.0 as i32, p1.1 as i32);
    let (x2, y2) = (p2.0 as i32, p2.1 as i32);

    // Calculate differences and steps
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Find GCD to get minimum step size
    let gcd = num::integer::gcd(dx.abs(), dy.abs());
    let step_x = dx / gcd;
    let step_y = dy / gcd;

    // Start from the first point and go backwards until we hit a boundary
    let mut x = x1;
    let mut y = y1;
    while x >= 0 && y >= 0 && (x as usize) < grid.row_len && (y as usize) < grid.col_len {
        points.push((x as usize, y as usize));
        x -= step_x;
        y -= step_y;
    }

    // Start from the first point again and go forwards until we hit a boundary
    x = x1 + step_x; // Start from next point since we already added x1,y1
    y = y1 + step_y;
    while x >= 0 && y >= 0 && (x as usize) < grid.row_len && (y as usize) < grid.col_len {
        points.push((x as usize, y as usize));
        x += step_x;
        y += step_y;
    }

    points
}

fn find_antinodes_part2(grid: &Grid<char>) -> HashSet<(usize, usize)> {
    // Group cells by frequency
    let mut freq_map: HashMap<char, Vec<Cell<char>>> = HashMap::new();
    for cell in grid.iter_cells() {
        if cell.value != '.' {
            freq_map.entry(cell.value).or_default().push(cell.clone());
        }
    }
    debug!("Freq map: {:?}", freq_map);

    let mut antinodes = HashSet::new();

    // For each frequency group
    for cells in freq_map.values() {
        // Skip frequencies with only one antenna
        if cells.len() < 2 {
            continue;
        }

        // Add all antenna positions as antinodes
        for cell in cells {
            antinodes.insert((cell.row, cell.col));
        }

        // For each pair of cells with same frequency
        for (i, cell1) in cells.iter().enumerate() {
            for cell2 in cells.iter().skip(i + 1) {
                // Find all points on the line between these antennas
                let points =
                    find_points_on_line((cell1.row, cell1.col), (cell2.row, cell2.col), grid);
                antinodes.extend(points);
            }
        }
    }

    antinodes
}

pub fn part1(fname: &str) -> u32 {
    let grid = read(fname);
    debug!("Grid: {:?}", grid);

    let antinodes = find_antinodes_part1(&grid);
    debug!("Antinodes: {:?}", antinodes);
    antinodes.len() as u32
}

pub fn part2(fname: &str) -> u32 {
    let grid = read(fname);
    debug!("Grid: {:?}", grid);

    let antinodes = find_antinodes_part2(&grid);
    debug!("Antinodes: {:?}", antinodes);
    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/08_sample.txt"), 14);
        assert_eq!(part1("data/08.txt"), 289);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/08_sample.txt"), 34);
        assert_eq!(part2("data/08.txt"), 1030);
    }
}
