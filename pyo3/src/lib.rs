use aoc_2024::day01;
use aoc_2024::day02;
use pyo3::prelude::*;

#[pyfunction]
fn day01_part1(filename: &str) -> PyResult<i32> {
    Ok(day01::part1(filename))
}

#[pyfunction]
fn day01_part2(filename: &str) -> PyResult<i32> {
    Ok(day01::part2(filename))
}

#[pyfunction]
fn day02_part1(filename: &str) -> PyResult<i32> {
    Ok(day02::part1(filename))
}

#[pyfunction]
fn day02_part2(filename: &str) -> PyResult<i32> {
    Ok(day02::part2(filename))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(day01_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day01_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day02_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day02_part2, m)?)?;
    Ok(())
}
