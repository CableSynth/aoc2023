mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    println!("Day 01");
    println!("Part 1: {}", day01::part1());
    println!("Part 2: {}", day01::part2());

    println!("Day 02");
    println!("Part 1: {}", day02::part1());
    println!("Part 2: {}", day02::part2());

    println!("Day 03");
    let day03_solution = day03::solution();
    println!("Part 1: {}", day03_solution.0);
    println!("Part 2: {}", day03_solution.1);

    println!("Day 04");
    let day04_solution = day04::solution();
    println!("Part 1: {}", day04_solution.0);
    println!("Part 2: {}", day04_solution.1);
}
