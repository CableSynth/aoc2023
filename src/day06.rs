use itertools::Itertools;
// const TEST_INPUT: &str = include_str!("../data/day06/test.txt");
const REAL_INPUT: &str = include_str!("../data/day06/input.txt");

pub fn solution() -> (u32, u32) {
    let (times, distance) = REAL_INPUT.lines().collect_tuple().unwrap();
    let times = times
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let distance = distance
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    println!("times: {:?}", times);
    println!("distance: {:?}", distance);

    let part1 = times.iter().zip(distance.iter()).fold(1, |acc, (t, d)| {
        let mut s = 0;
        for i in 1..*t {
            let td = (t - i) * i;
            if td > *d {
                s += 1;
            }
        }
        acc * s
    });

    let (times, distance) = REAL_INPUT.lines().collect_tuple().unwrap();

    let times = times
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect_vec()
        .concat()
        .parse::<usize>()
        .unwrap();
    let distance = distance
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect_vec()
        .concat()
        .parse::<usize>()
        .unwrap();
    let mut part2 = 0;
    for i in 1..times {
        let td = (times - i) * i;
        if td > distance {
            part2 += 1;
        }
    }
    (part1, part2)
}
