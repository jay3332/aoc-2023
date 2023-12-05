#![feature(never_type)]
#![allow(unused_variables)]

use std::collections::HashSet;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

fn count_winners(line: &str) -> usize {
    let (_, body) = line.split_once(": ").unwrap();
    let (winning, possessed) = body.split_once(" | ").unwrap();

    let winning = winning
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    possessed
        .split_ascii_whitespace()
        .filter(|s| winning.contains(&s.parse::<usize>().unwrap()))
        .count()
}

pub fn part_1(input: &'static str) -> Output<usize> {
    let sum = input
        .lines()
        .map(|line| 2_usize.pow(count_winners(line) as u32 - 1))
        .sum();

    Ok(sum)
}

pub fn part_2(input: &'static str) -> Output<usize> {
    let mut lines = input.lines();
    let mut counts = vec![1_usize; lines.clone().count()];

    for (i, line) in lines.enumerate() {
        let count = counts[i];
        counts[i + 1..i + count_winners(line) + 1]
            .iter_mut()
            .for_each(|c| *c += count);
    }
    Ok(counts.into_iter().sum())
}
