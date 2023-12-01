fn main() {
    let input: Vec<&'static str> = include_str!("../input")
        .split('\n')
        .filter(|s| s.chars().any(|c| !c.is_whitespace()))
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &[&'static str]) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        // Part 1 only wants us to find the literal characters 1-9, so I use `char.to_digit`,
        // alongside a `filtermap` to only get out the values which were successfully parsed
        let buf: Vec<u32> = Vec::from_iter(line.chars().filter_map(|c| c.to_digit(10)));

        if let (Some(first), Some(last)) = (buf.first(), buf.last()) {
            sum += (first * 10) + last;
        } else {
            panic!("[Part 1] Couldn't find any digits within line {line:?}!");
        }
    }
    sum
}

pub fn part2(input: &[&'static str]) -> u32 {
    let mut sum: u32 = 0;

    for line in input {
        let mut buf: Vec<u32> = Vec::with_capacity(line.len());

        // Part 2 now asks us to also consider the literal words for digits "one" through "nine",
        // so I made a helper function for trying to parse a digit value out of the start of
        // a string.
        for i in 0..line.len() {
            if let Some(val) = get_number_at_start(&line[i..]) {
                buf.push(val);
            }
        }

        if let (Some(first), Some(last)) = (buf.first(), buf.last()) {
            sum += (first * 10) + last;
        } else {
            panic!("[Part 2] Couldn't find any digits within line {line:?}!");
        }
    }

    sum
}

/// Given some string, try to parse the beginning to see if it begins with either a digit 1-9,
/// or the literal word "one" through to "nine"
fn get_number_at_start(s: &str) -> Option<u32> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, word) in words.into_iter().enumerate() {
        if s.starts_with(word) {
            return Some(i as u32 + 1);
        }
    }
    if let Some(c) = s.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(c);
    }
    None
}

#[cfg(test)]
mod day1_tests {
    use crate::{part1, part2};

    const TEST_INPUT_1: &[&'static str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    const TEST_INPUT_2: &[&'static str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    const EXPECTED_PART_1: u32 = 142;
    const EXPECTED_PART_2: u32 = 281;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), EXPECTED_PART_1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), EXPECTED_PART_2);
    }
}
