mod parsing;
use parsing::{parse_input, Coords, Galaxy};

fn main() {
    let raw_input = include_str!("../input");
    println!("Part 1: {}", part1(raw_input));
    println!("Part 2: {}", part2(raw_input));
}

fn part1(raw_input: &str) -> u64 {
    let galaxy = parse_input(raw_input, 2);
    sum_of_distances(&galaxy)
}
fn part2(raw_input: &str) -> u64 {
    let galaxy = parse_input(raw_input, 1_000_000);
    sum_of_distances(&galaxy)
}

fn sum_of_distances(galaxy: &Galaxy) -> u64 {
    let mut total = 0;
    for (i, g) in galaxy.iter().enumerate() {
        for (j, h) in galaxy.iter().enumerate() {
            if i < j {
                let dist = taxicab(g, h);
                // println!("Between galaxy {i} and {j}: {dist}");
                total += dist;
            }
        }
    }
    total
}

fn taxicab(lhs: &Coords, rhs: &Coords) -> u64 {
    lhs.x().abs_diff(rhs.x()) + lhs.y().abs_diff(rhs.y())
}
