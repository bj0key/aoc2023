fn main() {
    let input: Vec<&'static str> = include_str!("../input")
        .split('\n')
        .filter(|s| s.chars().any(|c| !c.is_whitespace()))
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&'static str>) {
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
    println!("Part 1: {sum}");
}

fn part2(input: &Vec<&'static str>) {
    let mut sum: u64 = 0;

    for line in input {
        let mut buf: Vec<u64> = Vec::with_capacity(line.len());

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

    println!("Part 2: {sum}");
}

/// Given some string, try to parse the beginning to see if it begins with either a digit 1-9,
/// or the literal word "one" through to "nine"
fn get_number_at_start(s: &str) -> Option<u64> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, word) in words.into_iter().enumerate() {
        if s.starts_with(word) {
            return Some(i as u64 + 1);
        }
    }
    if let Some(c) = s.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(c as u64);
    }
    None
}
