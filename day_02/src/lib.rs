#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<usize> {
    Ok(input
        .lines()
        .filter_map(|line| {
            // we can start at 6 because in the best case "Game X:", ':' is at index 6
            let colon = line[6..].find(':').unwrap() + 6;
            let game_id = line[5..colon].parse::<usize>().unwrap();

            line[colon + 2..]
                .split("; ")
                .all(|set| {
                    let set = set.as_bytes();

                    let mut cursor = 0;
                    let mut encountered = 0;
                    while cursor < set.len() {
                        // if we encounter a digit, consume until ' '
                        if encountered == 0 {
                            let start = cursor;
                            cursor += 1;
                            while set[cursor] != b' ' {
                                cursor += 1;
                            }
                            // SAFETY: input is assumed to follow a consistent format
                            let str = unsafe { std::str::from_utf8_unchecked(&set[start..cursor]) };
                            encountered = str.parse::<usize>().unwrap();
                            cursor += 1; // skip the whitespace
                            continue;
                        }
                        // if first case fails we are guaranteed to hit r/g/b. check if it falls
                        // within the cube limit, if so jump ahead
                        let color = set[cursor] as usize % 3;
                        // the following maps the limit as red => 12, green => 13, blue => 14
                        if encountered > color + 12 {
                            return false;
                        }
                        // corresponding lengths for red, green, blue + comma + space
                        cursor += [5, 7, 6][color];
                        encountered = 0;
                    }
                    true
                })
                .then_some(game_id)
        })
        .sum())
}

pub fn part_2(input: &'static str) -> Output<usize> {
    Ok(input
        .lines()
        .map(|line| {
            // we can start at 6 because in the best case "Game X:", ':' is at index 6
            let colon = line[6..].find(':').unwrap() + 6;
            let game_id = line[5..colon].parse::<usize>().unwrap();

            let mut seen = [0, 0, 0]; // [r, g, b]
            for set in line[colon + 2..].split("; ") {
                let set = set.as_bytes();

                let mut cursor = 0;
                let mut encountered = 0;
                while cursor < set.len() {
                    // if we encounter a digit, consume until ' '
                    if encountered == 0 {
                        let start = cursor;
                        cursor += 1;
                        while set[cursor] != b' ' {
                            cursor += 1;
                        }
                        // SAFETY: input is assumed to follow a consistent format
                        let str = unsafe { std::str::from_utf8_unchecked(&set[start..cursor]) };
                        encountered = str.parse::<usize>().unwrap();
                        cursor += 1; // skip the whitespace
                        continue;
                    }
                    // if first case fails we are guaranteed to hit r/g/b. check if it falls
                    // within the cube limit, if so jump ahead
                    let color = set[cursor] as usize % 3;
                    seen[color] = seen[color].max(encountered);
                    // corresponding lengths for red, green, blue + comma + space
                    cursor += [5, 7, 6][color];
                    encountered = 0;
                }
            }
            // compute power of the seen colors
            let [r, g, b] = seen;
            r * g * b
        })
        .sum())
}
