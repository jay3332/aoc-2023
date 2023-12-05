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

fn parse_expand_right(line: &str, origin: usize) -> usize {
    let mut cursor = origin;
    let bytes = line.as_bytes();
    while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
        cursor += 1;
    }
    line[origin..cursor].parse().unwrap()
}

fn parse_expand_left(line: &str, origin: usize) -> usize {
    let mut cursor = origin as isize;
    let bytes = line.as_bytes();
    while cursor >= 0 && bytes[cursor as usize].is_ascii_digit() {
        cursor -= 1;
    }
    line[(cursor + 1) as usize..origin + 1].parse().unwrap()
}

fn parse_expand_out(line: &str, origin: usize) -> usize {
    let (mut start, mut end) = (origin as isize, origin);
    let bytes = line.as_bytes();

    while start >= 0 && bytes[start as usize].is_ascii_digit() {
        start -= 1;
    }
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    line[(start + 1) as usize..end].parse().unwrap()
}

pub fn part_2(input: &'static str) -> Output<usize> {
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
            let chars = line.as_bytes();
            // this time, we're scanning for '*'
            line.char_indices().filter(|(_, c)| *c == '*').filter_map(|(idx, _)| {
                let mut ratios = [0, 0];
                let mut buffer = 0;

                macro_rules! check {
                    ($check:expr => $f:ident($e:ident, $idx:expr) $(, else $not_cond:expr => $not:expr)?) => {{
                        let mut result = false;
                        if $check.is_ascii_digit() {
                            ratios[buffer] = $f($e, $idx);
                            if buffer == 2 {
                                result = true;
                            }
                            buffer += 1;
                        }
                        $(
                            if !$check.is_ascii_digit() && $not_cond {
                                result = $not;
                            }
                        )?
                        result
                    }};
                }

                if idx > 0 && idx + 1 < width && (
                    check!(chars[idx - 1] => parse_expand_left(line, idx - 1))
                        || check!(chars[idx + 1] => parse_expand_right(line, idx + 1))
                ) {
                    return None;
                }

                let mut check_ext = |subject: &str| {
                    let bytes = subject.as_bytes();
                    check!(
                        bytes[idx] => parse_expand_out(subject, idx),
                        else idx > 0 => check!(
                            bytes[idx - 1] => parse_expand_left(subject, idx - 1)
                        ) || idx + 1 < width && check!(
                            bytes[idx + 1] => parse_expand_right(subject, idx + 1)
                        )
                    )
                };
                if check_ext(top) || check_ext(bottom) {
                    None
                } else {
                    let [left, right] = ratios;
                    Some(left * right)
                }
            })
            .sum::<usize>()
        })
        .sum();

    Ok(sum)
}
