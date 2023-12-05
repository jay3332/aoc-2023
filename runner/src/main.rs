fn main() {
    println!("=== AoC Day 03 Part 2 ===\n");
    let instant = std::time::Instant::now();
    let out = day_03::part_2(include_str!("../../input/day_03.txt"));

    match out {
        Ok(out) => println!("Output\n======\n{}\n", out),
        Err(err) => println!("Error\n=====\n{:?}\n", err),
    }
    println!("Execution time: {:?}", instant.elapsed());
}
