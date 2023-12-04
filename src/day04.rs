use std::iter::repeat;

use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day04/test.txt");
const REAL_INPUT: &str = include_str!("../data/day04/input.txt");

pub fn solution() -> (usize, usize) {
    let lines = REAL_INPUT.trim().lines().collect_vec();
    let mut card_num = repeat(1 as u32).take(lines.len()).collect_vec();
    let mut index = 0_usize;
    let cards = lines
        .iter()
        .filter_map(|l| {
            let (_, numbers) = l.split(":").collect_tuple().unwrap();
            let (wins, my_nums) = numbers.trim().split(" | ").collect_tuple().unwrap();
            let wins = wins
                .trim()
                .split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect_vec();
            let my_nums = my_nums
                .trim()
                .split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect_vec();
            let nums: u32 = wins
                .iter()
                .filter(|w| my_nums.contains(w))
                .count()
                .try_into()
                .unwrap();
            let i = if nums as usize + index > card_num.len() {
                (card_num.len() - index).checked_sub(1).unwrap_or_default()
            } else {
                nums as usize + index
            };
            // println!("i: {}", i);
            let undertest = card_num[index];
            card_num.get_mut((index + 1)..=i).unwrap_or(&mut[]).iter_mut().for_each(|c| *c += undertest );
            index += 1;
            // println!("{:?}", card_num);
            if nums == 0 {
                None
            } else {
                Some(nums)
            }
        })
        .collect_vec();

    // println!("{:?}", card_num);
    let part1 = cards
        .iter()
        .map(|c| 2_usize.pow((*c as i32 - 1).try_into().unwrap()))
        .sum();
    let part2 = card_num.iter().sum::<u32>() as usize;
    (part1, part2)
}
