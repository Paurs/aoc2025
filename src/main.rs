use aoc2025::day03::{part1, part2};
use aoc2025::utils;

fn main() {
    let input = utils::read_input("inputs/03.txt").unwrap();

    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}
