use std::fs::File;
use std::io::{self, BufRead};

pub trait FromLines {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>;
}
pub fn read_lines<T>(filename: &str) -> io::Result<T>
where
    T: FromLines,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines().filter_map(Result::ok);
    Ok(T::from_lines(lines))
}
