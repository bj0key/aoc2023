fn main() {
    let raw_input = include_str!("../input");
    let parsed_input = parse_input(raw_input);

    println!("Part 1: {}", predict(&parsed_input, Part::One));
    println!("Part 2: {}", predict(&parsed_input, Part::Two));
}

enum Part {
    One,
    Two,
}

fn predict(input: &[Vec<i64>], part: Part) -> i64 {
    let (diff_getter, diff_folder): (fn(&Vec<i64>) -> i64, fn(Vec<i64>) -> i64) = match part {
        Part::One => (
            |v| *v.last().unwrap(),
            |v| v.into_iter().fold(0, |acc, el| acc + el),
        ),
        Part::Two => (
            |v| *v.first().unwrap(),
            |v| -v.into_iter().rfold(0, |acc, el| el - acc),
        ),
    };

    let mut total = 0;
    for line in input.iter() {
        let mut diffs = vec![];
        let mut derived = derive(&line);
        while derived.iter().any(|n| n != &0) {
            diffs.push(diff_getter(&derived));
            derived = derive(&derived);
        }
        // let prediction = *line.first().unwrap() - diffs.into_iter().rfold(0, |acc, el| el - acc);
        let prediction = diff_getter(&line) + diff_folder(diffs);
        total += prediction;
    }
    total
}

fn derive(history: &[i64]) -> Vec<i64> {
    history.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}
