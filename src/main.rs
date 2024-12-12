use std::time::Instant;

fn main() {
    env_logger::init();

    let fname = "/root/aoc/data/day11_sample.txt";
    println!("Solving for {}", fname);
    let t0 = Instant::now();
    println!("Solution: {}", aoc_2024::day11::part1(fname));
    println!("Time: {:?}", Instant::now().duration_since(t0));
}
