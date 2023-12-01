use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../data/day01/test.txt");
const REAL_INPUT: &str = include_str!("../data/day01/input.txt");

fn read_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
    
}

pub fn part1() -> u32 {
    let lines = read_input(REAL_INPUT);
    lines.iter().map(|x| {
        let chr = x.chars().collect_vec();
        let first = x.find(|c: char| c.is_ascii_digit()).unwrap();
        let last = x.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let first_dig = chr[first].to_digit(10).unwrap();
        let last_dig = chr[last].to_digit(10).unwrap();
        first_dig*10 + last_dig


    }).collect::<Vec<u32>>().iter().sum()

 }
pub fn part2() -> u32 {
    let lines = read_input(REAL_INPUT);
    let rg = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    lines.iter().map(|x| {
        let m = rg.find_iter(&x).map(|x| x.as_str()).collect::<Vec<&str>>();
        let first_dig = m.first().unwrap();
        let first_dig = match *first_dig {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _=> first_dig.parse::<u32>().unwrap(),
        };
        let last_dig = m.last().unwrap();
        let last_dig = match *last_dig {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _=> last_dig.parse::<u32>().unwrap(),
        };
        first_dig*10 + last_dig

    }).collect::<Vec<u32>>().iter().sum()
}