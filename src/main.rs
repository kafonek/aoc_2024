use std::time::Instant;

mod day01;

fn main() {
    let fname = "data/01_sample.txt";
    let t0 = Instant::now();
    println!("Solution: {}", day01::part1(fname));
    println!("Time: {:?}", Instant::now().duration_since(t0));
}
