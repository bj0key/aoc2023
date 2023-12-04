fn main() {
    let raw_input = include_str!("../input");
    let parsed_input = parse_input(raw_input);
    println!("Day 1: {}", part1(&parsed_input));
    println!("Day 2: {}", part2(&parsed_input));
}

fn part1(cards: &[Scratchcard]) -> u32 {
    let mut total = 0;
    for card in cards {
        let winners = card_winners(card);
        if winners > 0 {
            total += 2_u32.pow(winners - 1);
        }
    }

    total
}

fn part2(cards: &[Scratchcard]) -> u32 {
    let mut card_counts: Vec<usize> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let winners = card_winners(card) as usize;
        for j in (i + 1)..(i + 1 + winners) {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.into_iter().sum::<usize>() as u32
}

fn card_winners(card: &Scratchcard) -> u32 {
    card.present
        .iter()
        .filter(|c| card.winning.contains(c))
        .count() as u32
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    let mut cards = vec![];
    for (present, winning) in input.lines().map(|s| {
        s.split_once(':')
            .and_then(|(_, s)| s.split_once('|'))
            .unwrap()
    }) {
        let [present, winning]: [Vec<u8>; 2] = [present, winning].map(|s| {
            s.split_ascii_whitespace()
                .filter_map(|s| s.parse::<u8>().ok())
                .collect()
        });
        cards.push(Scratchcard { present, winning });
    }
    cards
}
struct Scratchcard {
    present: Vec<u8>,
    winning: Vec<u8>,
}

#[cfg(test)]
mod day4_tests {
    use crate::{parse_input, part1, part2};

    const EXAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const EXPECTED_PART_1: u32 = 13;
    const EXPECTED_PART_2: u32 = 30;

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&parsed_input), EXPECTED_PART_1);
    }
    #[test]
    fn test_part2() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&parsed_input), EXPECTED_PART_2);
    }
}
