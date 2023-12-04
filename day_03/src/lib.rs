#![feature(never_type)]
#![feature(iter_map_windows)]
#![allow(unused_variables)]

use std::iter::once;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<usize> {
    let mut lines = input.lines().peekable();
    // SAFETY: we can assume that the input is valid, i.e. it contains at least one line.
    let width = unsafe { lines.peek().unwrap_unchecked().len() };
    let dummy = ".".repeat(width);

    // pad top and bottom with dummy lines
    let lines = once(&*dummy).chain(lines).chain(once(&*dummy));
    // we will "slide a window" of 3 lines over the input; this is why we needed to pad dummy lines
    // on top and bottom of the input as we will only ever be processing the middle line
    let sum = lines
        .map_windows::<_, _, 3>(|&[top, line, bottom]| {
            #[inline(always)]
            fn check_punctuation(haystack: &str) -> bool {
                haystack
                    .chars()
                    .any(|c| c != '.' && c.is_ascii_punctuation())
            }

            // find groups of digits
            let chars = line.as_bytes();
            let mut cursor = 0_usize;
            let mut total = 0_usize;

            while cursor < chars.len() {
                if !chars[cursor].is_ascii_digit() {
                    cursor += 1;
                    continue;
                }

                // if we encounter a digit, determine its right bound
                let start = cursor;
                let left_bound = start.saturating_sub(1); // since diagonals work
                cursor += 1;
                while cursor < chars.len() && chars[cursor].is_ascii_digit() {
                    cursor += 1;
                }
                // is this adjacent to any symbol, except for '.'?
                //
                // note that we can assume that left and right bounds are NOT digits since we already
                // checked for that
                let right_bound = line.len().min(cursor + 1);
                if check_punctuation(&top[left_bound..right_bound])
                    || check_punctuation(&line[left_bound..right_bound])
                    || check_punctuation(&bottom[left_bound..right_bound])
                {
                    total += line[start..cursor].parse::<usize>().unwrap();
                }
                cursor += 1;
            }
            total
        })
        .sum();

    Ok(sum)
}

pub fn part_2(input: &'static str) -> Output<!> {
    todo!()
}
