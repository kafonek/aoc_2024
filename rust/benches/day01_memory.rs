use aoc_2024::day01;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    println!("{}", day01::part1("data/01.txt"));

    let stats = dhat::HeapStats::get();
    println!("Heap stats:");
    println!("{:#?}", stats);
}
