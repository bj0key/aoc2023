fn main() {
    let raw_input = include_str!("../input");

    let p1_parsed_input = part1_parse(raw_input);
    println!("Part 1: {}", part1(&p1_parsed_input));

    let p2_parsed_input = part2_parse(&raw_input);
    println!("Part 2: {}", part2(&p2_parsed_input));
}

fn part1(races: &[Race]) -> i64 {
    let mut total = 1;
    for race in races {
        let (root1, root2) = winning_hold_durations(race);
        let range = root2 - root1 + 1;
        total *= range;
    }
    total
}

fn part2(race: &Race) -> i64 {
    let (root1, root2) = winning_hold_durations(race);
    root2 - root1 + 1
}

fn winning_hold_durations(race: &Race) -> (i64, i64) {
    // For a given race of duration `duration`ms, holding the button for `hold`ms will result in
    // moving hold * (duration - hold) mm, AKA -hold^2 + duration*hold
    // Our goal is to find the range of distances where that duration is > the current record,
    // AKA -hold^2 + duration*hold > record,
    // AKA -hold^2 + duration*hold - record > 0
    // I see a quadratic equation!!!
    quadratic(-1, race.duration, -race.record)
}

fn quadratic(a: i64, b: i64, c: i64) -> (i64, i64) {
    let sqrt_discrimnant = (((b * b) - (4 * a * c)) as f64).sqrt();
    let root1 = ((-b as f64) + sqrt_discrimnant) / (2 * a) as f64;
    let root2 = ((-b as f64) - sqrt_discrimnant) / (2 * a) as f64;
    (root1.floor() as i64 + 1, root2.ceil() as i64 - 1)
}

struct Race {
    duration: i64,
    record: i64,
}

fn part1_parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times_line = lines.next().and_then(|s| s.strip_prefix("Time:")).unwrap();
    let distances_line = lines
        .next()
        .and_then(|s| s.strip_prefix("Distance:"))
        .unwrap();
    let [times, distances] = [times_line, distances_line].map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
    });
    times
        .zip(distances)
        .map(|(time, distance)| Race {
            duration: time,
            record: distance,
        })
        .collect()
}

fn part2_parse(input: &str) -> Race {
    let mut lines = input.lines();
    let [time, dist]: [String; 2] = [lines.next().unwrap(), lines.next().unwrap()]
        .map(|line| line.chars().filter(char::is_ascii_digit).collect());

    Race {
        duration: time.parse().unwrap(),
        record: dist.parse().unwrap(),
    }
}
