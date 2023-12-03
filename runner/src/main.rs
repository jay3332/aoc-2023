fn main() {
    println!("=== AoC Day 01 Part 2 ===\n");
    let instant = std::time::Instant::now();
    let out = day_01::part_2(include_str!("../../input/day_01.txt"));

    match out {
        Ok(out) => println!("Output\n======\n{}\n", out),
        Err(err) => println!("Error\n=====\n{:?}\n", err),
    }
    println!("Execution time: {:?}", instant.elapsed());
}
