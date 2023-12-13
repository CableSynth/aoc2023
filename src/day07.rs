use std::collections::BinaryHeap;

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
        get_hand_strength(self);
        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
    
}

fn get_hand_strength(hand: &Hand) -> usize {
    let card_by_count = hand.cards.iter().counts();
    println!("{:?}", card_by_count  );
    let counts = card_by_count.values().collect_vec();

    println!("{:?}", counts  );
    let mut cards = hand.cards.to_vec();
    let mut strength = 0;
    let strength = cards.iter().map(|c| card_strength(*c)).collect_vec();
    match(*counts.iter().max().unwrap_or(&0)) {
        5 => 5
    }
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

    let cards = TEST_INPUT.lines().map(|l| {
        let (hand, bid) = l.split_once(" ").unwrap();
        Hand {
            cards: hand.chars().collect_vec().try_into().unwrap(),
            bid: bid.parse().unwrap()
        }
    }).collect_vec();
    let cards: BinaryHeap<Hand> = cards.into();
    println!("vec deq {:?}", cards);
    println!("top {:?}", cards.iter().next());
    let part1 = 0;
    let part2 = 0;

    (part1, part2)
}