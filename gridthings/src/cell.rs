#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Cell<T: Copy> {
    pub value: T,
    pub y: i32,
    pub x: i32,
}

impl<T: Copy> Cell<T> {
    pub fn new(value: T, y: i32, x: i32) -> Self {
        Self { value, y, x }
    }
}
