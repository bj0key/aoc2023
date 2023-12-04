fn main() {
    let raw_input = include_str!("../input");
    let parsed_input = parse_input(raw_input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

fn part1(schematic: &Schematic) -> u32 {
    // Find all numbers which are neighbours with a symbol
    let mut numbers: Vec<(usize, u32)> = vec![];
    for (y, row) in schematic.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Component::Number { id, value } = cell {
                if any_neighbour_symbols(&schematic, x, y) {
                    numbers.push((*id, *value));
                }
            }
        }
    }
    numbers.sort();
    // Dedup to remove duplicate IDs, to prevent double-counting
    numbers.dedup();
    numbers.into_iter().map(|(_, val)| val).sum()
}

fn part2(schematic: &Schematic) -> u32 {
    let mut num_pairs: Vec<Vec<u32>> = vec![];
    // Find all '*'s which have 2 surrounding numbers, and put those nums in a big list
    for (y, row) in schematic.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Component::Symbol('*') = cell {
                let neighbor_nums = neighbour_nums(schematic, x, y);
                if neighbor_nums.len() == 2 {
                    num_pairs.push(neighbor_nums);
                }
            }
        }
    }
    num_pairs
        .into_iter()
        .map(|v| v.into_iter().product::<u32>())
        .sum()
}

fn any_neighbour_symbols(schematic: &Schematic, x: usize, y: usize) -> bool {
    assert!(match schematic[y][x] {
        Component::Number { .. } => true,
        _ => false,
    });

    for (x, y) in neighbours(x, y) {
        if let Some(Component::Symbol(_)) = schematic.get(y).and_then(|v| v.get(x)) {
            return true;
        }
    }
    false
}

/// Gets a Vec of any numbers surrounding a set of coords, already de-duplicated
fn neighbour_nums(schematic: &Schematic, x: usize, y: usize) -> Vec<u32> {
    assert!(match schematic[y][x] {
        Component::Symbol('*') => true,
        _ => false,
    });
    let mut nums = vec![];
    for (x, y) in neighbours(x, y) {
        if let Some(Component::Number { id, value }) = schematic.get(y).and_then(|v| v.get(x)) {
            nums.push((id, value));
        }
    }
    nums.sort();
    nums.dedup();
    Vec::from_iter(nums.into_iter().map(|(_, n)| *n))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Component {
    Nothing,
    Symbol(char),
    Number { id: usize, value: u32 },
}

type Schematic = Vec<Vec<Component>>;

fn parse_input(input: &str) -> Schematic {
    let mut v: Vec<Vec<Component>> = vec![];
    let mut id = 0;
    for (_, line) in input.lines().enumerate() {
        let mut row = vec![];
        let mut digits_to_skip = 0;
        for (x, chr) in line.chars().enumerate() {
            if digits_to_skip > 0 {
                // For skipping over the digits of numbers we just parsed
                digits_to_skip -= 1;
                continue;
            } else if chr.is_ascii_digit() {
                // For parsing numbers
                let value = line[x..]
                    .chars()
                    .map_while(|c| c.to_digit(10))
                    .fold(0, |agg, i| 10 * agg + i);

                let number_length = value.ilog10() + 1;
                for _ in 0..number_length {
                    row.push(Component::Number { id, value });
                }
                id += 1;
                digits_to_skip = number_length - 1;
            } else if chr == '.' {
                // For parsing nothingness
                row.push(Component::Nothing);
            } else {
                // All else are symbols
                row.push(Component::Symbol(chr));
            }
        }
        v.push(row)
    }

    #[cfg(debug_assertions)]
    {
        let len = v[0].len();
        assert!(v.iter().all(|v| v.len() == len));
    }

    v
}

/// Get all neighbouring pairs of coords. Returns pairs of the form (x, y)
fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for i in (x.saturating_sub(1))..=(x.saturating_add(1)) {
        for j in (y.saturating_sub(1))..=(y.saturating_add(1)) {
            if i != x || j != y {
                v.push((i, j));
            }
        }
    }
    v
}

#[cfg(test)]
mod day3_tests {
    use crate::{parse_input, part1, part2};

    const EXAMPLE_INPUT_DATA: &'static str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    const EXPECTED_PART_1: u32 = 4361;

    const EXPECTED_PART_2: u32 = 467835;

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_INPUT_DATA);
        assert_eq!(part1(&input), EXPECTED_PART_1)
    }
    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_INPUT_DATA);
        assert_eq!(part2(&input), EXPECTED_PART_2)
    }
}
