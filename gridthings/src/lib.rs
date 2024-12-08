#[derive(Clone, Debug)]
pub struct Grid<T: Copy> {
    pub data: Vec<T>,
    pub row_len: usize,
    pub col_len: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Cell<T: Copy> {
    pub value: T,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    DiagonalUpRight,
    DiagonalDownRight,
    DiagonalUpLeft,
    DiagonalDownLeft,
}

impl Direction {
    // Order is clockwise starting from right
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Right,
            Direction::DiagonalDownRight,
            Direction::Down,
            Direction::DiagonalDownLeft,
            Direction::Left,
            Direction::DiagonalUpLeft,
            Direction::Up,
            Direction::DiagonalUpRight,
        ]
    }

    pub fn cardinal() -> Vec<Direction> {
        vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }
}

impl<T: Copy> Grid<T> {
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

    pub fn get_value(&self, row: usize, col: usize) -> Option<T> {
        if row >= self.col_len || col >= self.row_len {
            return None;
        }
        Some(self.data[row * self.row_len + col])
    }

    pub fn update_cell_value(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.row_len + col] = value;
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.get_value(row, col)
            .map(|value| Cell { value, row, col })
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = Cell<T>> + '_ {
        (0..self.col_len).flat_map(move |row| {
            (0..self.row_len).map(move |col| Cell {
                value: self.get_value(row, col).unwrap(),
                row,
                col,
            })
        })
    }

    pub fn get_cell_neighbor(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<Cell<T>> {
        let (y_step, x_step) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::DiagonalUpRight => (-1, 1),
            Direction::DiagonalDownRight => (1, 1),
            Direction::DiagonalUpLeft => (-1, -1),
            Direction::DiagonalDownLeft => (1, -1),
        };

        // Convert to i32 for signed arithmetic
        let row = row as i32;
        let col = col as i32;

        let dst_row = row + y_step;
        let dst_col = col + x_step;

        // Check for negative values or out of bounds
        if dst_row < 0 || dst_col < 0 {
            return None;
        }

        self.get_cell(dst_row as usize, dst_col as usize)
    }

    pub fn get_cell_neighbors(
        &self,
        row: usize,
        col: usize,
        directions: Vec<Direction>,
    ) -> Vec<Cell<T>> {
        directions
            .iter()
            .map(|direction| self.get_cell_neighbor(row, col, direction.clone()))
            .filter_map(|cell| cell)
            .collect()
    }

    pub fn get_row(&self, row: usize) -> Vec<Cell<T>> {
        (0..self.row_len)
            .map(|col| self.get_cell(row, col).unwrap())
            .collect()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<Cell<T>>> + '_ {
        (0..self.col_len).map(move |row| self.get_row(row))
    }

    pub fn get_segment(
        &self,
        row_start: usize,
        col_start: usize,
        direction: Direction,
        length: usize,
    ) -> Option<Vec<T>> {
        let (y_step, x_step) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::DiagonalUpRight => (-1, 1),
            Direction::DiagonalDownRight => (1, 1),
            Direction::DiagonalUpLeft => (-1, -1),
            Direction::DiagonalDownLeft => (1, -1),
        };

        let mut segment = Vec::new();
        for i in 0..length {
            let row = row_start as isize + i as isize * y_step;
            let col = col_start as isize + i as isize * x_step;

            if row < 0 || col < 0 || row >= self.col_len as isize || col >= self.row_len as isize {
                return None;
            }

            let value = self.get_value(row as usize, col as usize);
            if value.is_none() {
                return None;
            }
            segment.push(value.unwrap());
        }
        Some(segment)
    }

    pub fn all_segments(&self, row_start: usize, col_start: usize, length: usize) -> Vec<Vec<T>> {
        let mut results = Vec::new();
        for direction in Direction::all() {
            let segment = match self.get_segment(row_start, col_start, direction, length) {
                Some(segment) => segment,
                None => continue,
            };
            results.push(segment);
        }
        results
    }

    pub fn subgrid(&self, row: usize, col: usize, height: usize, width: usize) -> Option<Grid<T>> {
        if row + height > self.col_len || col + width > self.row_len {
            return None;
        }

        let mut data = Vec::with_capacity(height * width);
        for r in row..row + height {
            for c in col..col + width {
                data.push(self.data[r * self.row_len + c]);
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

    #[rustfmt::skip]
    #[test]
    fn test_get_neighbors() {
        let grid = make_i32_grid();
        assert_eq!(
            grid.get_cell_neighbors(0, 0, Direction::all()),
            vec![
                Cell { value: 2, row: 0, col: 1 },
                Cell { value: 5, row: 1, col: 1 },
                Cell { value: 4, row: 1, col: 0 }
            ]
        );

        assert_eq!(
            grid.get_cell_neighbors(0, 0, Direction::cardinal()),
            vec![
                Cell { value: 2, row: 0, col: 1 },
                Cell { value: 4, row: 1, col: 0 }
            ]
        );
    }

    #[test]
    fn test_iter_rows() {
        let grid = make_i32_grid();
        let rows = grid.iter_rows().collect::<Vec<_>>();
        assert_eq!(rows.len(), 3);
        assert_eq!(
            rows[0][0],
            Cell {
                value: 1,
                row: 0,
                col: 0
            }
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_get_segment() {
        let grid = make_i32_grid();
        assert_eq!(grid.get_segment(1, 1, Direction::Up, 1), Some(vec![5]));
        assert_eq!(grid.get_segment(1, 1, Direction::Up, 2), Some(vec![5, 2]));
        assert_eq!(grid.get_segment(1, 1, Direction::Up, 3), None);
        assert_eq!(grid.get_segment(1, 1, Direction::Down, 2), Some(vec![5, 8]));
        assert_eq!(grid.get_segment(1, 1, Direction::Left, 2), Some(vec![5, 4]));
        assert_eq!(grid.get_segment(1, 1, Direction::Right, 2), Some(vec![5, 6]));
        assert_eq!(grid.get_segment(1, 1, Direction::DiagonalUpRight, 2), Some(vec![5, 3]));
        assert_eq!(grid.get_segment(1, 1, Direction::DiagonalDownRight, 2), Some(vec![5, 9]));
        assert_eq!(grid.get_segment(1, 1, Direction::DiagonalDownLeft, 2), Some(vec![5, 7]));
        assert_eq!(grid.get_segment(1, 1, Direction::DiagonalUpLeft, 2), Some(vec![5, 1]));
    }

    #[test]
    fn test_all_segments() {
        let grid = make_i32_grid();
        assert_eq!(
            grid.all_segments(0, 0, 3),
            vec![vec![1, 2, 3], vec![1, 5, 9], vec![1, 4, 7]]
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
}
