fn main() {
    let raw_input = include_str!("../input");
    let parsed_input = parse_input(raw_input);

    prettyprint_almanac(&parsed_input);

    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

fn lowest_location<'a>(seeds: impl Iterator<Item = i64>, map_groups: &[MapGroup]) -> i64 {
    let mut min_location = i64::MAX;

    for seed in seeds {
        let mut curr_item = seed;
        for map_group in map_groups {
            let next_item = map_group
                .map_ranges
                .iter()
                .filter_map(|range| range.try_map(curr_item))
                .next()
                .unwrap_or(curr_item);

            curr_item = next_item;
            // item_type = &map_group.value_type;
        }
        if curr_item < min_location {
            min_location = curr_item;
        }
    }
    min_location
}

fn part1(almanac: &Almanac) -> i64 {
    lowest_location(almanac.seeds.iter().map(|n| *n), &almanac.map_groups)
}

fn part2(almanac: &Almanac) -> i64 {
    // Uh-oh, the seeds are actually ranges!!
    // This is super-bruteforce, but in release mode completes in ~90s on my machine.
    let real_seeds = almanac.seeds.chunks_exact(2).flat_map(|c| {
        debug_assert!(c.len() == 2);
        c[0]..(c[0] + c[1])
    });
    lowest_location(real_seeds, &almanac.map_groups)
}

struct Almanac {
    seeds: Vec<i64>,
    map_groups: Vec<MapGroup>,
}

struct MapGroup {
    value_type: String,
    key_type: String,
    map_ranges: Vec<MapRange>,
}

struct MapRange {
    dest_start: i64,
    src_start: i64,
    length: i64,
}
impl MapRange {
    #[inline]
    fn contains_in_src(&self, val: i64) -> bool {
        self.src_start <= val && val <= self.src_start + self.length
    }
    #[inline]
    fn diff(&self) -> i64 {
        self.dest_start - self.src_start
    }
    fn try_map(&self, val: i64) -> Option<i64> {
        if self.contains_in_src(val) {
            Some(val + self.diff())
        } else {
            None
        }
    }
}

fn prettyprint_almanac(almanac: &Almanac) {
    println!("Seeds: {:?}", almanac.seeds);
    for group in almanac.map_groups.iter() {
        println!("  {} to {} ranges:", group.key_type, group.value_type);
        for range in group.map_ranges.iter() {
            println!(
                "    Dest {}, Src {}, Length {}",
                range.dest_start, range.src_start, range.length
            );
        }
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut grouping_iter = input.split("\n\n");

    let seeds_str = grouping_iter.next().unwrap();
    let seeds: Vec<i64> = seeds_str
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map_groups = vec![];
    for grouping in grouping_iter {
        let (mapping_label, data) = grouping.split_once(" map:\n").unwrap();

        // Parsing the "from" and "to" of the mapping
        let (key_type, value_type) = mapping_label.split_once("-to-").unwrap();
        let from_type = key_type.to_string();
        let to_type = value_type.to_string();

        let mut map_ranges = vec![];
        for line in data.lines() {
            let mut num_iter = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap());
            match (num_iter.next(), num_iter.next(), num_iter.next()) {
                // Each line is of the form [destination start], [source start], [range length]
                (Some(dest_start), Some(src_start), Some(length)) => {
                    map_ranges.push(MapRange {
                        dest_start,
                        src_start,
                        length,
                    });
                }
                _ => unreachable!(),
            }
        }

        map_groups.push(MapGroup {
            value_type: to_type,
            key_type: from_type,
            map_ranges,
        });
    }

    Almanac { seeds, map_groups }
}

#[cfg(test)]
mod day5_tests {
    use crate::{parse_input, part1, part2};

    // If testing, ensure that day5/ contains the example-input file!
    const EXAMPLE_INPUT: &'static str = include_str!("../example-input");

    const EXPECTED_PART_1: i64 = 35;
    const EXPECTED_PART_2: i64 = 46;

    #[test]
    fn test_part1() {
        let almanac = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&almanac), EXPECTED_PART_1);
    }
    #[test]
    fn test_part2() {
        let almanac = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&almanac), EXPECTED_PART_2);
    }
}
