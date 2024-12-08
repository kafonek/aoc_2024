use crate::{cell::Cell, direction::Direction};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Segment<T: Copy> {
    pub values: Vec<T>,
    pub cells: Vec<Cell<T>>,
    pub direction: Direction,
}

impl<T: Copy> Segment<T> {
    pub fn new(values: Vec<T>, cells: Vec<Cell<T>>, direction: Direction) -> Self {
        Self {
            values,
            cells,
            direction,
        }
    }

    pub fn start(&self) -> Option<&Cell<T>> {
        self.cells.first()
    }

    pub fn end(&self) -> Option<&Cell<T>> {
        self.cells.last()
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}
