use std::collections::HashMap;

use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day08/test.txt");
const REAL_INPUT: &str = include_str!("../data/day08/input.txt");

pub fn solution() -> (usize, usize) {
    let (instructions, mapping) = parse_input(REAL_INPUT);
    // let real_input = parse_input(REAL_INPUT);
    (find(instructions.as_bytes(), b"AAA", b"ZZZ", mapping.clone()), 0)
}

fn parse_input(input: &str) -> (&str, HashMap<&[u8], (&[u8], &[u8])>) {
    let (instructions, mapping) = input.split_once("\n\n").unwrap();
    let map = mapping.trim().lines().map(|l| {
        let line = l.as_bytes();
        
        (&line[0..3], (&line[7..10], &line[12..15]))
    }).collect::<HashMap<_, _>>();
    (instructions, map)
}

fn find(instructions: &[u8], start: &[u8], end: &[u8], mapping: HashMap<&[u8], (&[u8], &[u8])>) -> usize {
    let mut current = start;
    1 + instructions.iter().cycle().position(|&c| {
        current = if c == b'L' {
            mapping.get(current).unwrap().0
        } else {
            mapping.get(current).unwrap().1
        };
        current.ends_with(end)
    }).unwrap()
    
}
