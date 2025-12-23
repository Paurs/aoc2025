pub fn part1(input: Vec<String>) -> usize {
    let mut splitted = input.split(|line| line.is_empty());

    let range_section = splitted.next().unwrap();
    let id_section = splitted.next().unwrap();

    let mut ranges = range_section
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    ranges.sort();

    id_section
        .iter()
        .filter(|line| {
            let id = line.parse().unwrap();
            ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(&id))
        })
        .count()
}

pub fn part2(input: Vec<String>) -> usize {
    let mut splitted = input.split(|line| line.is_empty());

    let mut ranges = splitted
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    ranges.sort();

    let mut merged = vec![ranges[0]];

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();

        if start > last.1 {
            merged.push((start, end));
        } else {
            last.1 = last.1.max(end);
        }
    }

    merged.iter().map(|&(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day05::{part1, part2},
        utils::read_example,
    };

    #[test]
    fn test_1() {
        let input = read_example(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_2() {
        let input = read_example(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(input), 14);
    }
}
