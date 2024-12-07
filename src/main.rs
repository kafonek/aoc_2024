use std::time::Instant;

fn main() {
    env_logger::init();

    let fname = "data/07.txt";
    let t0 = Instant::now();
    println!("Solution: {}", aoc_2024::day07::part2(fname));
    println!("Time: {:?}", Instant::now().duration_since(t0));
}
