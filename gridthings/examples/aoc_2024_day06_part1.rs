use gridthings::{Direction, Grid};
use std::{collections::HashSet, time::Instant};

const INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

fn solve() -> i32 {
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

    // Find starting position (^)
    let mut cell = grid.iter_cells().find(|c| c.value == '^').unwrap();
    println!("Start Cell: {:?}", cell);

    let mut visited = HashSet::new();
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut directions = directions.iter().cycle();
    let mut direction = directions.next().unwrap();

    loop {
        visited.insert(cell.clone());
        let next_cell = grid.get_cell_neighbor(cell.y, cell.x, direction.clone());

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

fn main() {
    let start_time = Instant::now();

    let answer = solve();
    println!("Visited {} unique positions", answer);
    println!("Time taken: {:?}", start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 41);
    }
}
