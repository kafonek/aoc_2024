use std::time::Instant;

fn main() {
    env_logger::init();

    let fname = "data/02.txt";
    let t0 = Instant::now();
    println!("Solution: {}", aoc_2024::day02::part2(fname));
    println!("Time: {:?}", Instant::now().duration_since(t0));
}
