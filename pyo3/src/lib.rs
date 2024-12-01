use aoc_2024::day01;
use pyo3::prelude::*;

#[pyfunction]
fn part1(filename: &str) -> PyResult<i32> {
    Ok(day01::part1(filename))
}

#[pyfunction]
fn part2(filename: &str) -> PyResult<i32> {
    Ok(day01::part2(filename))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(part1, m)?)?;
    m.add_function(wrap_pyfunction!(part2, m)?)?;
    Ok(())
}
