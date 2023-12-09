use std::collections::HashMap;

fn main() {
    let raw_input = include_str!("../input");

    let (directions, map) = parse_input(&raw_input);

    println!("Part 1: {}", part1(&directions, &map));
    println!("Part 2: {}", part2(&directions, &map));
}

fn part1(directions: &[Direction], map: &Network) -> i64 {
    path_length(b"AAA", |loc| loc == b"ZZZ", directions, map)
}

fn part2(directions: &[Direction], map: &Network) -> i64 {
    let starter_locations: Vec<&Location> = map.keys().filter(|k| k[2] == b'A').collect();
    let mut lengths = vec![];
    for loc in starter_locations {
        lengths.push(path_length(loc, |loc| loc[2] == b'Z', directions, map));
    }
    lengths.into_iter().fold(1, |acc, r| lcm(acc, r))
}

fn lcm(l: i64, r: i64) -> i64 {
    l / gcd(l, r) * r
}

fn gcd(l: i64, r: i64) -> i64 {
    if r == 0 {
        l
    } else {
        gcd(r, l % r)
    }
}

fn path_length(
    start: &Location,
    end_condition: fn(&Location) -> bool,
    directions: &[Direction],
    map: &Network,
) -> i64 {
    let mut current_location: &Location = start;
    let mut dir_iter = directions.iter().cycle();
    let mut steps = 0;
    while !end_condition(current_location) {
        let direction = dir_iter.next().unwrap();
        let node = map.get(current_location).unwrap();
        current_location = match direction {
            Direction::Left => &node.0,
            Direction::Right => &node.1,
        };
        steps += 1;
    }

    steps
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
type Location = [u8; 3];
type Network = HashMap<Location, (Location, Location)>;

fn parse_input(input: &str) -> (Vec<Direction>, Network) {
    let (direction, nodes) = input.split_once("\n\n").unwrap();

    let directions = direction
        .bytes()
        .map(|b| match b {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    let network = nodes
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            // 0..3, 7..10, 12..15
            let [node_name, lhs, rhs] =
                [0..3, 7..10, 12..15].map(|range| bytes[range].try_into().unwrap());

            (node_name, (lhs, rhs))
        })
        .collect();

    (directions, network)
}
