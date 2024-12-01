use aoc_2024::day01::Day01Part1;
use aoc_2024::utils::read_lines;

fn main() -> std::io::Result<()> {
    let t0 = std::time::Instant::now();
    let data: Day01Part1 = read_lines("data/01_01.txt")?;
    let result = data.similarity();
    println!("Total similarity: {}", result);
    println!("Time taken: {:?}", t0.elapsed());
    Ok(())
}
