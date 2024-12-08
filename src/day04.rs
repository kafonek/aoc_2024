use gridthings::Grid;

fn read(fname: &str) -> Grid<char> {
    let content = std::fs::read_to_string(fname).unwrap();
    let data: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    Grid::from_vecs(&data)
}

pub fn part1(fname: &str) -> i32 {
    let grid = read(fname);
    let mut xmas_matches = 0;
    for cell in grid.iter_cells() {
        let segments = grid.all_segments(cell.y, cell.x, 4);
        for segment in segments {
            if segment.values == vec!['X', 'M', 'A', 'S'] {
                xmas_matches += 1;
            }
        }
    }
    xmas_matches
}

fn is_x_mas_grid(grid: &Grid<char>) -> bool {
    if grid.data[4] != 'A' {
        return false;
    }
    for idx in [0, 2, 6, 8] {
        if grid.data[idx] != 'M' && grid.data[idx] != 'S' {
            return false;
        }
    }
    // assert that 0, 8 and 2,6 are not the same
    if grid.data[0] == grid.data[8] || grid.data[2] == grid.data[6] {
        return false;
    }
    true
}

pub fn part2(fname: &str) -> i32 {
    let grid = read(fname);
    let mut xmas_matches = 0;
    for cell in grid.iter_cells() {
        let subgrid = grid.subgrid(cell.y, cell.x, 3, 3);
        if subgrid.is_none() {
            continue;
        }
        let subgrid = subgrid.unwrap();
        if is_x_mas_grid(&subgrid) {
            xmas_matches += 1;
        }
    }
    xmas_matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/04_01_sample.txt"), 18);
        assert_eq!(part1("data/04.txt"), 2646);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/04_02_sample.txt"), 9);
        assert_eq!(part2("data/04.txt"), 2000);
    }
}
