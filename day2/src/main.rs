use std::str::FromStr;

fn main() {
    let raw_input = include_str!("../input");
    let parsed_input = parse_input(raw_input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

pub fn part1(games: &[Game]) -> u32 {
    let mut total: u32 = 0;
    for game in games {
        if game
            .draws
            .iter()
            .all(|(color, num)| color.max_allowed() >= *num)
        {
            total += game.id;
        }
    }
    total
}

pub fn part2(games: &[Game]) -> u32 {
    let mut total: u32 = 0;
    for game in games {
        let mut maxes = [0, 0, 0];
        for (color, n) in game.draws.iter() {
            let max = &mut maxes[*color as usize];
            if *n > *max {
                *max = *n;
            }
        }
        let product: u32 = maxes.into_iter().product();
        total += product;
    }
    total
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn max_allowed(&self) -> u32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

pub struct Game {
    id: u32,
    draws: Vec<(Color, u32)>,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    let mut games = vec![];
    for line in input.lines() {
        let (game_id_str, bag_pulls_str) =
            line.strip_prefix("Game ").unwrap().split_once(':').unwrap();

        let game_id: u32 = game_id_str.parse().unwrap();

        let mut game_draws = vec![];

        for color_chunk in bag_pulls_str.split(|c| c == ',' || c == ';').map(str::trim) {
            let (n_str, color_str) = color_chunk.split_once(' ').unwrap();
            let n: u32 = n_str.parse().unwrap();
            let color: Color = color_str.parse().unwrap();
            game_draws.push((color, n));
        }

        games.push(Game {
            id: game_id,
            draws: game_draws,
        });
    }
    games
}

#[cfg(test)]
mod day2_tests {
    use crate::{parse_input, part1, part2};

    const EXAMPLE_INPUT: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const EXPECTED_PART_1: u32 = 8;

    const EXPECTED_PART_2: u32 = 2286;

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
