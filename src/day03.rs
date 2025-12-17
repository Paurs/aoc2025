pub fn part1(input: Vec<String>) -> u64 {
    let mut voltage = 0;

    for s in input {
        let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

        let (max_idx, &max) = digits[..(digits.len() - 1)]
            .iter()
            .enumerate()
            .max_by_key(|(idx, &val)| (val, usize::MAX - idx))
            .unwrap();

        let second = digits[(max_idx + 1)..].iter().max().unwrap();

        voltage += ((max * 10) + second) as u64;
    }

    voltage
}

#[allow(unused)]
pub fn part2(input: Vec<String>) -> u64 {
    let mut voltage = 0;

    for s in input {
        let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut stack = Vec::new();

        for (i, digit) in digits.iter().enumerate() {
            while !stack.is_empty()
                && digit > stack[stack.len() - 1]
                && stack.len() + (digits.len() - i) > 12
            {
                stack.pop();
            }

            if stack.len() < 12 {
                stack.push(digit);
            }
        }
        voltage += stack.iter().fold(0, |acc, &&digit| acc * 10 + digit as u64);
    }

    voltage
}

#[cfg(test)]
mod tests {
    use crate::{
        day03::{part1, part2},
        utils::read_example,
    };

    #[test]
    fn test_1() {
        let input =
            read_example("987654321111111\n811111111111119\n234234234234278\n818181911112111")
                .iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_2() {
        let input =
            read_example("987654321111111\n811111111111119\n234234234234278\n818181911112111")
                .iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(part2(input), 3121910778619);
    }
}
