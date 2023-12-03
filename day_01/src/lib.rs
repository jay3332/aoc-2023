#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &str) -> Output<u32> {
    Ok(input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter_map(|c| c.to_digit(10));
            // SAFETY: we can assume that the input is valid, i.e. it contains at least one digit.
            let first = unsafe { chars.next().unwrap_unchecked() };
            first * 10 + chars.last().unwrap_or(first)
        })
        .sum())
}

pub fn part_2(input: &str) -> Output<u32> {
    // move the input to a mutable buffer
    let mut input = input.as_bytes().to_vec();

    #[inline(always)]
    fn fast_replace<const N: usize>(input: &mut [u8], from: [u8; N], to: u8) {
        let mut i = 0;
        let max_i = input.len() - N;
        while i < max_i {
            i += if input[i..i + N] == from {
                input[i + 1] = to;
                N
            } else {
                1
            }
        }
    }

    {
        let buffer = &mut input;
        fast_replace(buffer, *b"one", b'1');
        fast_replace(buffer, *b"two", b'2');
        fast_replace(buffer, *b"three", b'3');
        fast_replace(buffer, *b"four", b'4');
        fast_replace(buffer, *b"five", b'5');
        fast_replace(buffer, *b"six", b'6');
        fast_replace(buffer, *b"seven", b'7');
        fast_replace(buffer, *b"eight", b'8');
        fast_replace(buffer, *b"nine", b'9');
    }

    // SAFETY: input is derived from valid UTF-8.
    part_1(unsafe { std::str::from_utf8_unchecked(&input) })
}
