use regex::Regex;

pub fn part1(input: Vec<String>) -> usize {
    let re = Regex::new(r"^([LR])(\d{1,3})$").unwrap();

    let mut pos: i32 = 50;
    let mut password: usize = 0;

    for s in input.iter() {
        if let Some(captures) = re.captures(s) {
            let direction = captures.get(1).unwrap().as_str();
            let number: i32 = captures
                .get(2)
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .unwrap();

            let delta = if direction == "L" {
                number * -1
            } else {
                number
            };

            pos = (pos + delta).rem_euclid(100);

            if pos == 0 {
                password += 1;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn part2(input: Vec<String>) -> usize {
    let re = Regex::new(r"^([LR])(\d{1,9})$").unwrap();

    let mut pos: i32 = 50;
    let mut password: usize = 0;

    for s in input.iter() {
        if let Some(captures) = re.captures(s) {
            let direction = captures.get(1).unwrap().as_str();
            let mut number: i32 = captures
                .get(2)
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .unwrap();

            let delta = if direction == "L" {
                number * -1
            } else {
                number
            };

            if delta >= 0 {
                password += ((pos + delta) / 100) as usize;
            } else {
                password += ((((100 - pos) % 100) - delta) / 100) as usize;
            }

            pos = (pos + delta).rem_euclid(100);
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use crate::{
        day01::{part1, part2},
        utils::read_example,
    };

    #[test]
    fn test_1() {
        let input = read_example("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82")
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_2() {
        let input = read_example("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82")
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part2(input), 6);
    }
}
