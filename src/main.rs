use aoc2025::day02::{self, part1, part2};
use aoc2025::utils;

fn main() {
    let input = utils::read_input("inputs/02.txt").unwrap();

    let parsed = day02::parse(input.first().unwrap());

    println!("Part1: {}", part1(parsed.clone()));
    println!("Part2: {}", part2(parsed));
}
