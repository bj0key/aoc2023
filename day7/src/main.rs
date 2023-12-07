fn main() {
    let raw_input = include_str!("../input");
    let parsed_input_pt1 = parse_input(raw_input, false);
    let parsed_input_pt2 = parse_input(raw_input, true);

    println!("Part 1: {}", total_winnings(&parsed_input_pt1));
    println!("Part 1: {}", total_winnings(&parsed_input_pt2));
}

fn total_winnings(input: &[(Hand, i32)]) -> i32 {
    let mut hand_bet_pairs = input.to_vec();
    let mut total = 0;
    hand_bet_pairs.sort_by(|(c1, _), (c2, _)| c1.cmp(c2));
    for (i, (_, bet)) in hand_bet_pairs.iter().enumerate() {
        total += (1 + i as i32) * *bet;
    }
    total
}

fn parse_input(input: &str, for_part2: bool) -> Vec<(Hand, i32)> {
    let card_parsing_fn = if for_part2 {
        Card::from_char_pt2
    } else {
        Card::from_char
    };

    let mut pairs = vec![];
    for (hand_s, bet_s) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        // Stable rust doesn't have Iter::next_chunk yet :(
        let hand_vec: Vec<Card> = hand_s.chars().map(card_parsing_fn).collect();
        let mut hand = [Card::Ace; 5];
        hand.copy_from_slice(&hand_vec[0..5]);

        let bet: i32 = bet_s.parse().unwrap();
        pairs.push((Hand(hand), bet));
    }

    pairs
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Card {
    const fn amount() -> usize {
        14
    }
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid cast to Card: {c}"),
        }
    }
    /// Nearly identical to `from_char`, except Js get mapped to Joker instead of Jack
    fn from_char_pt2(c: char) -> Self {
        match Self::from_char(c) {
            Self::Jack => Self::Joker,
            other => other,
        }
    }
}

type CardCounts = [u8; Card::amount()];

#[derive(Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandType {
    fn from_counts(mut counts: CardCounts) -> Self {
        let joker_count = counts[Card::Joker as usize];
        let counts = &mut counts[1..];

        counts.sort();
        let mut top_2_max = [counts[counts.len() - 1], counts[counts.len() - 2]];
        // So it turns out that the way to make the best hand is to always make the joker cards
        // represent whatever number has the otherwise highest count in the hand.
        top_2_max[0] += joker_count;

        match top_2_max {
            [5, 0] => Self::FiveOfAKind,
            [4, 1] => Self::FourOfAKind,
            [3, 2] => Self::FullHouse,
            [3, 1] => Self::ThreeOfAKind,
            [2, 2] => Self::TwoPair,
            [2, 1] => Self::OnePair,
            [1, 1] => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Hand([Card; 5]);
impl Hand {
    /// Tallies up the cards into an array of counts.
    fn counts(&self) -> CardCounts {
        let mut counts: CardCounts = [0; Card::amount()];
        for card in self.0 {
            counts[card as usize] += 1;
        }
        counts
    }
    fn hand_type(&self) -> HandType {
        HandType::from_counts(self.counts())
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let l_type = self.hand_type() as u8;
        let r_type = other.hand_type() as u8;

        if l_type < r_type {
            return Ordering::Less;
        } else if l_type > r_type {
            return Ordering::Greater;
        }

        // Else, l_type == r_type, do the tiebreaker logic
        for (l_card, r_card) in self.0.iter().zip(other.0.iter()) {
            if l_card < r_card {
                return Ordering::Less;
            } else if l_card > r_card {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}
// These 3 impls all just rely on the Ord impl
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}
impl Eq for Hand {}
