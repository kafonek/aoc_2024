use crate::{cell::Cell, direction::Direction, segment::Segment};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Grid<T: Copy> {
    pub data: Vec<T>,
    pub row_len: usize,
    pub col_len: usize,
}

#[derive(Debug)]
pub struct GridParseError {
    pub row: usize,
    pub col: usize,
    pub value: String,
    pub target_type: String,
}

impl std::fmt::Display for GridParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse grid: character '{}' at position (row {}, col {}) could not be parsed as {}",
            self.value, self.row, self.col, self.target_type
        )
    }
}

impl std::error::Error for GridParseError {}

impl<T: Copy + FromStr> Grid<T> {
    pub fn from_str(input: &str) -> Result<Self, GridParseError> {
        let mut data = Vec::new();
        for (row_idx, line) in input.trim().lines().enumerate() {
            let mut row = Vec::new();
            for (col_idx, c) in line.chars().enumerate() {
                match c.to_string().parse() {
                    Ok(value) => row.push(value),
                    Err(_) => {
                        return Err(GridParseError {
                            row: row_idx,
                            col: col_idx,
                            value: c.to_string(),
                            target_type: std::any::type_name::<T>().to_string(),
                        })
                    }
                }
            }
            data.push(row);
        }
        Ok(Self::from_vecs(&data))
    }

    pub fn from_vecs(vecs: &[Vec<T>]) -> Self {
        let data = vecs.iter().flat_map(|vec| vec.iter().cloned()).collect();
        let row_len = vecs[0].len();
        let col_len = vecs.len();
        Self {
            data,
            row_len,
            col_len,
        }
    }

    fn is_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && (y as usize) < self.col_len && (x as usize) < self.row_len
    }

    pub fn get_value(&self, y: i32, x: i32) -> Option<T> {
        if !self.is_in_bounds(y, x) {
            return None;
        }
        Some(self.data[y as usize * self.row_len + x as usize])
    }

    pub fn update_cell_value(&mut self, y: i32, x: i32, value: T) -> Option<()> {
        if !self.is_in_bounds(y, x) {
            return None;
        }
        self.data[y as usize * self.row_len + x as usize] = value;
        Some(())
    }

    pub fn get_cell(&self, y: i32, x: i32) -> Option<Cell<T>> {
        self.get_value(y, x).map(|value| Cell::new(value, y, x))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = Cell<T>> + '_ {
        (0..self.col_len as i32).flat_map(move |y| {
            (0..self.row_len as i32).map(move |x| Cell::new(self.get_value(y, x).unwrap(), y, x))
        })
    }

    pub fn get_cell_neighbor(&self, y: i32, x: i32, direction: Direction) -> Option<Cell<T>> {
        let (y_step, x_step) = direction.delta();
        self.get_cell(y + y_step, x + x_step)
    }

    pub fn get_cell_neighbors(&self, y: i32, x: i32, directions: Vec<Direction>) -> Vec<Cell<T>> {
        directions
            .iter()
            .filter_map(|direction| self.get_cell_neighbor(y, x, direction.clone()))
            .collect()
    }

    pub fn get_row(&self, y: i32) -> Option<Vec<Cell<T>>> {
        if !self.is_in_bounds(y, 0) {
            return None;
        }
        Some(
            (0..self.row_len as i32)
                .map(|x| self.get_cell(y, x).unwrap())
                .collect(),
        )
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<Cell<T>>> + '_ {
        (0..self.col_len as i32).map(move |y| self.get_row(y).unwrap())
    }

    pub fn get_segment(
        &self,
        y_start: i32,
        x_start: i32,
        direction: Direction,
        length: usize,
    ) -> Option<Segment<T>> {
        let (y_step, x_step) = direction.delta();

        let mut values = Vec::new();
        let mut cells = Vec::new();
        for i in 0..length {
            let y = y_start + i as i32 * y_step;
            let x = x_start + i as i32 * x_step;

            let cell = self.get_cell(y, x)?;
            values.push(cell.value);
            cells.push(cell);
        }
        Some(Segment::new(values, cells, direction))
    }

    pub fn all_segments(&self, y_start: i32, x_start: i32, length: usize) -> Vec<Segment<T>> {
        Direction::all()
            .into_iter()
            .filter_map(|direction| self.get_segment(y_start, x_start, direction, length))
            .collect()
    }

    pub fn subgrid(&self, y: i32, x: i32, height: usize, width: usize) -> Option<Grid<T>> {
        if !self.is_in_bounds(y, x)
            || !self.is_in_bounds(y + height as i32 - 1, x + width as i32 - 1)
        {
            return None;
        }

        let mut data = Vec::with_capacity(height * width);
        for r in y..(y + height as i32) {
            for c in x..(x + width as i32) {
                data.push(self.data[r as usize * self.row_len + c as usize]);
            }
        }

        Some(Grid {
            data,
            row_len: width,
            col_len: height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // util make a char grid
    fn make_char_grid() -> Grid<char> {
        Grid::from_vecs(&[
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ])
    }

    fn make_i32_grid() -> Grid<i32> {
        Grid::from_vecs(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    }

    #[test]
    fn test_char_grid_from_vecs() {
        let grid = make_char_grid();
        assert_eq!(grid.data, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
        assert_eq!(grid.row_len, 3);
        assert_eq!(grid.col_len, 3);
    }

    #[test]
    fn test_i32_grid_from_vecs() {
        let grid = make_i32_grid();
        assert_eq!(grid.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid.row_len, 3);
        assert_eq!(grid.col_len, 3);
    }

    #[test]
    fn test_update_cell_value() {
        let mut grid = make_i32_grid();
        assert_eq!(grid.get_value(0, 0), Some(1));
        grid.update_cell_value(0, 0, 10);
        assert_eq!(grid.get_value(0, 0), Some(10));
    }

    #[test]
    fn test_get_neighbors() {
        let grid = make_i32_grid();
        assert_eq!(
            grid.get_cell_neighbors(0, 0, Direction::all()),
            vec![Cell::new(2, 0, 1), Cell::new(5, 1, 1), Cell::new(4, 1, 0)]
        );

        assert_eq!(
            grid.get_cell_neighbors(0, 0, Direction::cardinal()),
            vec![Cell::new(2, 0, 1), Cell::new(4, 1, 0)]
        );
    }

    #[test]
    fn test_iter_rows() {
        let grid = make_i32_grid();
        let rows = grid.iter_rows().collect::<Vec<_>>();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0], Cell::new(1, 0, 0));
    }

    #[test]
    fn test_get_segment() {
        let grid = make_i32_grid();
        assert_eq!(
            grid.get_segment(1, 1, Direction::Up, 1),
            Some(Segment::new(
                vec![5],
                vec![Cell::new(5, 1, 1)],
                Direction::Up
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::Up, 2),
            Some(Segment::new(
                vec![5, 2],
                vec![Cell::new(5, 1, 1), Cell::new(2, 0, 1)],
                Direction::Up
            ))
        );
        assert_eq!(grid.get_segment(1, 1, Direction::Up, 3), None);
        assert_eq!(
            grid.get_segment(1, 1, Direction::Down, 2),
            Some(Segment::new(
                vec![5, 8],
                vec![Cell::new(5, 1, 1), Cell::new(8, 2, 1)],
                Direction::Down
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::Left, 2),
            Some(Segment::new(
                vec![5, 4],
                vec![Cell::new(5, 1, 1), Cell::new(4, 1, 0)],
                Direction::Left
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::Right, 2),
            Some(Segment::new(
                vec![5, 6],
                vec![Cell::new(5, 1, 1), Cell::new(6, 1, 2)],
                Direction::Right
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::DiagonalUpRight, 2),
            Some(Segment::new(
                vec![5, 3],
                vec![Cell::new(5, 1, 1), Cell::new(3, 0, 2)],
                Direction::DiagonalUpRight
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::DiagonalDownRight, 2),
            Some(Segment::new(
                vec![5, 9],
                vec![Cell::new(5, 1, 1), Cell::new(9, 2, 2)],
                Direction::DiagonalDownRight
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::DiagonalDownLeft, 2),
            Some(Segment::new(
                vec![5, 7],
                vec![Cell::new(5, 1, 1), Cell::new(7, 2, 0)],
                Direction::DiagonalDownLeft
            ))
        );
        assert_eq!(
            grid.get_segment(1, 1, Direction::DiagonalUpLeft, 2),
            Some(Segment::new(
                vec![5, 1],
                vec![Cell::new(5, 1, 1), Cell::new(1, 0, 0)],
                Direction::DiagonalUpLeft
            ))
        );
    }

    #[test]
    fn test_all_segments() {
        let grid = make_i32_grid();
        assert_eq!(
            grid.all_segments(0, 0, 3),
            vec![
                Segment::new(
                    vec![1, 2, 3],
                    vec![Cell::new(1, 0, 0), Cell::new(2, 0, 1), Cell::new(3, 0, 2)],
                    Direction::Right
                ),
                Segment::new(
                    vec![1, 5, 9],
                    vec![Cell::new(1, 0, 0), Cell::new(5, 1, 1), Cell::new(9, 2, 2)],
                    Direction::DiagonalDownRight
                ),
                Segment::new(
                    vec![1, 4, 7],
                    vec![Cell::new(1, 0, 0), Cell::new(4, 1, 0), Cell::new(7, 2, 0)],
                    Direction::Down
                )
            ]
        );
    }

    #[test]
    fn test_subgrid() {
        let grid = make_i32_grid();

        // Test valid 2x2 subgrid from top-left
        let subgrid = grid.subgrid(0, 0, 2, 2).unwrap();
        assert_eq!(subgrid.data, vec![1, 2, 4, 5]);
        assert_eq!(subgrid.row_len, 2);
        assert_eq!(subgrid.col_len, 2);

        // Test valid 2x2 subgrid from middle
        let subgrid = grid.subgrid(1, 1, 2, 2).unwrap();
        assert_eq!(subgrid.data, vec![5, 6, 8, 9]);
        assert_eq!(subgrid.row_len, 2);
        assert_eq!(subgrid.col_len, 2);

        // Test out of bounds
        let subgrid = grid.subgrid(2, 2, 2, 2);
        assert!(subgrid.is_none());
    }

    #[test]
    fn test_grid_from_str() {
        // Valid char grid
        let grid: Result<Grid<char>, GridParseError> = Grid::from_str("123\n456\n789");
        assert!(grid.is_ok());

        // Valid i32 grid
        let grid: Result<Grid<i32>, GridParseError> = Grid::from_str("123\n456\n789");
        assert!(grid.is_ok());

        // Invalid i32 grid (contains non-digit)
        let grid: Result<Grid<i32>, GridParseError> = Grid::from_str("123\n456\n7x9");
        assert!(grid.is_err());
    }

    #[test]
    fn test_out_of_bounds() {
        let grid = Grid::from_vecs(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        // Test negative coordinates
        assert_eq!(grid.get_cell(-1, 0), None);
        assert_eq!(grid.get_cell(0, -1), None);
        assert_eq!(grid.get_cell(-1, -1), None);

        // Test beyond bounds
        assert_eq!(grid.get_cell(3, 0), None);
        assert_eq!(grid.get_cell(0, 3), None);
        assert_eq!(grid.get_cell(3, 3), None);

        // Test valid coordinates
        assert_eq!(grid.get_cell(0, 0), Some(Cell::new(1, 0, 0)));
        assert_eq!(grid.get_cell(2, 2), Some(Cell::new(9, 2, 2)));
    }
}
