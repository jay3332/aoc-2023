fn main() {
    println!("=== AoC Day 04 Part 2 ===\n");
    let instant = std::time::Instant::now();
    let out = day_04::part_2(include_str!("../../input/day_04.txt"));

    match out {
        Ok(out) => println!("Output\n======\n{}\n", out),
        Err(err) => println!("Error\n=====\n{:?}\n", err),
    }
    println!("Execution time: {:?}", instant.elapsed());
}
