use std::collections::VecDeque;

use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day07/test.txt");
const REAL_INPUT: &str = include_str!("../data/day07/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
    bid: usize
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
    
}

fn get_hand_strength(hand: &Hand) -> usize {
    let mut cards = hand.cards.to_vec();
    let mut strength = 0;
    let strength = cards.iter().map(|c| card_strength(*c)).collect_vec();

    0
}

fn card_strength(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize
    }
}

pub fn solution() -> (usize, usize) {

    let mut cards = TEST_INPUT.lines().map(|l| {
        let (hand, bid) = l.split_once(" ").unwrap();
        (hand.chars(), bid.parse::<usize>().unwrap())
    }).collect_vec();
    let part1 = 0;
    let part2 = 0;

    (part1, part2)
}