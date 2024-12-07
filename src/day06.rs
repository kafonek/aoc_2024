use gridthings::{Cell, Direction, Grid};
use log::debug;
use rayon::prelude::*;
use std::{collections::HashSet, fs::read_to_string};

fn read(fname: &str) -> Grid<char> {
    let content = read_to_string(fname).unwrap();
    let data: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    Grid::from_vecs(&data)
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn get_next_cell(
    grid: &Grid<char>,
    cell: &Cell<char>,
    direction: &Direction,
) -> Option<Cell<char>> {
    match direction {
        // get_cell takes usize so need to check if we're having x or y go to negative here...
        Direction::Up if cell.row == 0 => None,
        Direction::Left if cell.col == 0 => None,
        Direction::Up => grid.get_cell(cell.row - 1, cell.col),
        Direction::Right => grid.get_cell(cell.row, cell.col + 1),
        Direction::Down => grid.get_cell(cell.row + 1, cell.col),
        Direction::Left => grid.get_cell(cell.row, cell.col - 1),
        _ => unreachable!(),
    }
}

pub fn part1(fname: &str) -> i32 {
    let grid = read(fname);
    let mut cell = grid.iter_cells().find(|c| c.value == '^').unwrap();
    debug!(
        "start cell (value {}): {}, {}",
        cell.value, cell.col, cell.row
    );

    let mut visited = HashSet::new();
    let mut directions = DIRECTIONS.iter().cycle();
    let mut direction = directions.next().unwrap();

    loop {
        visited.insert(cell.clone());
        let next_cell = get_next_cell(&grid, &cell, &direction);
        if next_cell.is_none() {
            break;
        }
        let next_cell = next_cell.unwrap();
        match next_cell.value {
            '#' => direction = directions.next().unwrap(),
            _ => cell = next_cell,
        }
    }
    visited.len() as i32
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Observation {
    cell: Cell<char>,
    direction: Direction,
}

fn grid_is_looped(grid: &Grid<char>, mut cell: Cell<char>) -> bool {
    let mut visited = HashSet::new();
    let mut directions = DIRECTIONS.iter().cycle();
    let mut direction = directions.next().unwrap();

    loop {
        let observation = Observation {
            cell: cell.clone(),
            direction: direction.clone(),
        };
        if visited.contains(&observation) {
            return true;
        }
        let next_cell = get_next_cell(&grid, &cell, &direction);
        if next_cell.is_none() {
            break;
        }
        let next_cell = next_cell.unwrap();
        match next_cell.value {
            '#' => direction = directions.next().unwrap(),
            _ => {
                visited.insert(observation);
                cell = next_cell;
            }
        }
    }
    false
}

pub fn part2(fname: &str) -> i32 {
    let grid = read(fname);
    let start_cell = grid.iter_cells().find(|cell| cell.value == '^').unwrap();
    debug!("start_cell: {:?}", start_cell);

    grid.iter_cells()
        .par_bridge()
        .filter_map(|cell| {
            if cell.value == '.' {
                let mut modified_grid = grid.clone();
                *modified_grid.get_mut_value(cell.row, cell.col).unwrap() = '#';
                grid_is_looped(&modified_grid, start_cell.clone()).then_some(1)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/06_sample.txt"), 41);
        assert_eq!(part1("data/06.txt"), 5153);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/06_sample.txt"), 6);
        assert_eq!(part2("data/06.txt"), 1711);
    }
}
