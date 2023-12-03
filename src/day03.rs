use std::collections::HashMap;

use itertools::Itertools;
// const TEST_INPUT: &str = include_str!("../data/day03/test.txt");
const REAL_INPUT: &str = include_str!("../data/day03/input.txt");

pub fn solution() -> (usize, usize) {
    let lines = REAL_INPUT
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect_vec();

    let mut sybmols = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut char_index = 0;
        while char_index < line.len() {
            let (start, mut symbol) = (char_index, None);
            while char_index < line.len() && line[char_index].is_ascii_digit() {
                symbol = symbol.or_else(|| symbol_at(&lines, line_num, char_index));
                char_index += 1;
            }
            if let Some(symbol) = symbol {
                let num = line[start..char_index]
                    .iter()
                    .fold(0, |n, c| n * 10 + (c - b'0') as usize);
                sybmols.entry(symbol).or_insert(Vec::new()).push(num);
            }
            char_index += 1;
        }
    }
    let part1 = sybmols.values().flatten().sum();
    let part2 = sybmols
        .iter()
        .filter(|(&(_, _, s), v)| s == '*' && v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();
    (part1, part2)
}

fn symbol_at(lines: &[&[u8]], line_num: usize, char_index: usize) -> Option<(usize, usize, char)> {
    for (dline, dchar) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (nline, nchar) = (
            (line_num as i32 + dline) as usize,
            (char_index as i32 + dchar) as usize,
        );
        let Some(&b) = lines
            .get(nline as usize)
            .and_then(|l| l.get(nchar as usize))
        else {
            continue;
        };
        if b != b'.' && !b.is_ascii_digit() {
            return Some((nchar as usize, nline as usize, b as char));
        }
    }
    None
}
