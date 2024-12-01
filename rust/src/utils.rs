use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait FromLines {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>;
}

pub fn read_lines<T>(filename: &str) -> io::Result<T>
where
    T: FromLines,
{
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../data")
        .join(filename);
    let file = File::open(path.clone()).unwrap_or_else(|_| panic!("File not found: {}", path.display()));
    let lines = io::BufReader::new(file).lines().filter_map(Result::ok);
    Ok(T::from_lines(lines))
}
