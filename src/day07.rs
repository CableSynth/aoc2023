use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day07/test.txt");
const REAL_INPUT: &str = include_str!("../data/day07/input.txt");

fn get_hand_strength(cards: &str, is_part2: bool) -> (usize, usize) {
    let card_by_count = cards.chars().counts();
    println!("{:?}", card_by_count);
    let counts = card_by_count
        .iter()
        .filter(|&(&k, _)| k != 'J' || !is_part2)
        .map(|(_, &v)| v)
        .collect_vec();
    let jack_count = if is_part2 {
        *card_by_count.get(&'J').unwrap_or(&0)
    } else {
        0
    };
    // This is a cool idea, shift 4 bits for each card, then add the card strength thus a second rank sort
    let idx = cards
        .chars()
        .fold(0, |acc, c| (acc << 4) + card_strength(c, is_part2));
    println!("idx: {}", idx);
    let hand_rank = match (*counts.iter().max().unwrap_or(&&0_usize), jack_count) {
        (c, j) if c + j == 5 => 6,
        (c, j) if c + j == 4 => 5,
        (3, 0) => {
            if counts.contains(&&2) {
                4
            } else {
                3
            }
        }
        (2, _) => {
            let pairs = counts.iter().filter(|&&c| c == 2).count();
            match (pairs, jack_count) {
                (2, 1) => 4,
                (1, 1) => 3,
                (2, 0) => 2,
                _ => 1,
            }
        }
        (1, 2) => 3,
        (1, 1) => 1,
        _ => 0,
    };
    (hand_rank, idx)
}

fn card_strength(card: char, is_part2: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if is_part2 {
                0
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}

pub fn solution() -> (usize, usize) {
    let mut cards = REAL_INPUT
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            (
                bid.parse().unwrap(),
                get_hand_strength(hand, false),
                get_hand_strength(hand, true),
            )
        })
        .collect_vec();
    cards.sort_unstable_by_key(|&(_, k, _)| k);
    let part1 = cards
        .iter()
        .enumerate()
        .map(|(i, (bid, _, _))| bid * (i + 1))
        .sum::<usize>();
    cards.sort_unstable_by_key(|&(_, _, k)| k);
    let part2 = cards
        .iter()
        .enumerate()
        .map(|(i, (bid, _, _))| bid * (i + 1))
        .sum::<usize>();

    (part1, part2)
}
