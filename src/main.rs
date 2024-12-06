use std::time::Instant;

fn main() {
    env_logger::init();

    let fname = "data/06_sample.txt";
    let t0 = Instant::now();
    println!("Solution: {}", aoc_2024::day06::part1(fname));
    println!("Time: {:?}", Instant::now().duration_since(t0));
}
