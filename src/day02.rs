pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|r| r.split_once("-").unwrap())
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect()
}

const DIVISORS_PART1: [(u64, u64, u64); 5] = [
    (10, 99, 11),
    (1000, 9999, 101),
    (100000, 999999, 1001),
    (10000000, 99999999, 10001),
    (1000000000, 9999999999, 100001),
];

const DIVISORS_PART2: [(u64, u64, &[u64]); 9] = [
    (10, 99, &[11]),
    (100, 999, &[111]),
    (1000, 9999, &[101, 1111]),
    (10000, 99999, &[11111]),
    (100000, 999999, &[1001, 10101, 111111]),
    (1000000, 9999999, &[1111111]),
    (10000000, 99999999, &[10001, 1010101, 11111111]),
    (100000000, 999999999, &[111111111, 1001001]),
    (1000000000, 9999999999, &[100001, 101010101, 1111111111]),
];

pub fn part1(input: Vec<(u64, u64)>) -> u64 {
    input
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&num| {
            DIVISORS_PART1.iter().any(|pattern| {
                if pattern.0 > num {
                    return false;
                }

                num <= pattern.1 && num % pattern.2 == 0
            })
        })
        .sum()
}

pub fn part2(input: Vec<(u64, u64)>) -> u64 {
    input
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&num| {
            DIVISORS_PART2.iter().any(|pattern| {
                if pattern.0 > num {
                    return false;
                }

                num <= pattern.1 && pattern.2.iter().any(|d| num % d == 0)
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day02::{parse, part1, part2};

    #[test]
    fn test_1() {
        let input = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );

        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_2() {
        let input = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );

        for (lo, hi) in input.clone() {
            println!("{} - {}", lo, hi);
        }

        assert_eq!(part2(input), 4174379265);
    }
}
