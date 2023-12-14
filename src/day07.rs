use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day07/test.txt");
const REAL_INPUT: &str = include_str!("../data/day07/input.txt");

fn get_hand_strength(cards: &str) -> (usize, usize) {
    let card_by_count = cards.chars().counts();
    println!("{:?}", card_by_count  );
    let counts = card_by_count.values().collect_vec();
    
    // This is a cool idea, shift 4 bits for each card, then add the card strength thus a second rank sort
    let idx = cards.chars().fold(0, |acc, c| {
        let temp = (acc << 4) + card_strength(c);
        println!("temp: {}", temp);
        temp
    });
    println!("idx: {}", idx);
    let hand_rank = match *counts.iter().max().unwrap_or(&&0_usize) {
        5 => 6,
        4 => 5,
        3 => {
            if counts.contains(&&2) {
                4
            } else {
                3
            }
        },
        2 => {
            let pairs = counts.iter().filter(|c| *c == &&2).count();
            match pairs {
                2 => 2,
                _ => 1,
            }
        },
        _ => 0
    };
    (hand_rank, idx)
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
        (bid.parse().unwrap(), get_hand_strength(hand))
    }).collect_vec();
    cards.sort_unstable_by_key(|&(_, k)| k);
    let part1 = cards.iter().enumerate().map(|(i, (bid, _))| {
        bid * (i + 1)
    }).sum::<usize>();
    let part2 = 0;

    (part1, part2)
}