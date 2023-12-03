use itertools::Itertools;
// const TEST_INPUT: &str = include_str!("../data/day02/test.txt");
const REAL_INPUT: &str = include_str!("../data/day02/input.txt");

pub fn part1() -> u32 {
    REAL_INPUT.trim().lines().map(parse_game).sum()
}

pub fn part2() -> u32 {
    REAL_INPUT.trim().lines().map(parse_game2).sum()
}

fn parse_game(l: &str) -> u32 {
    let split_colon = l.split(": ").collect_vec();
    let game_id = split_colon[0].trim().split_whitespace().last().unwrap();
    let game_id = game_id.parse::<u32>().unwrap();
    let game = split_colon[1].trim().split("; ").collect_vec();
    let match_possible = game
        .iter()
        .map(|x| {
            let split_comma = x.split(", ").collect_vec();
            let res = split_comma
                .iter()
                .map(|c| {
                    let (num, color) = c.split_whitespace().collect_tuple().unwrap();
                    part1_match(num, color)
                })
                .collect_vec();
            res.iter().all(|&x| x == true)
        })
        .collect_vec()
        .iter()
        .all(|&x| x == true);
    if match_possible {
        game_id
    } else {
        0
    }
}
fn parse_game2(l: &str) -> u32 {
    let split_colon = l.split(": ").collect_vec();
    let game = split_colon[1].trim().split("; ").collect_vec();
    let (mut red, mut green, mut blue) = (0, 0, 0);
    game
        .iter()
        .map(|x| {
            let split_comma = x.split(", ").collect_vec();
            split_comma
                .iter()
                .map(|c| {
                    let (num, color) = c.split_whitespace().collect_tuple().unwrap();
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => {
                            if num > red {
                                red = num;
                            }
                        }

                        "green" => {
                            if num > green {
                                green = num;
                            }
                        }
                        "blue" => {
                            if num > blue {
                                blue = num;
                            }
                        }
                        _ => unreachable!("Invalid color"),
                    };
                })
                .collect_vec();
        })
        .collect_vec();
    red * green * blue
}

fn part1_match(num: &str, color: &str) -> bool {
    let num = num.parse::<u32>().unwrap();
    match color {
        "red" => {
            if num > 12 {
                false
            } else {
                true
            }
        }
        "green" => {
            if num > 13 {
                false
            } else {
                true
            }
        }
        "blue" => {
            if num > 14 {
                false
            } else {
                true
            }
        }
        _ => unreachable!("Invalid color"),
    }
}