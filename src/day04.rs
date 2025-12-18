pub fn part1(input: Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut accessible = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let mut neighbour_count = 0;

                if x > 0 && y > 0 && grid[y - 1][x - 1] == '@' {
                    neighbour_count += 1;
                }
                if y > 0 && grid[y - 1][x] == '@' {
                    neighbour_count += 1;
                }
                if x < grid[y].len() - 1 && y > 0 && grid[y - 1][x + 1] == '@' {
                    neighbour_count += 1;
                }
                if x > 0 && grid[y][x - 1] == '@' {
                    neighbour_count += 1;
                }
                if x < grid[y].len() - 1 && grid[y][x + 1] == '@' {
                    neighbour_count += 1;
                }
                if x > 0 && y < grid.len() - 1 && grid[y + 1][x - 1] == '@' {
                    neighbour_count += 1;
                }
                if y < grid.len() - 1 && grid[y + 1][x] == '@' {
                    neighbour_count += 1;
                }
                if x < grid[y].len() - 1 && y < grid.len() - 1 && grid[y + 1][x + 1] == '@' {
                    neighbour_count += 1;
                }

                if neighbour_count < 4 {
                    accessible += 1;
                }
            }
        }
    }

    accessible
}

#[allow(unused)]
pub fn part2(input: Vec<String>) -> usize {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut removed = 0;
    let mut accessible = 1;

    while accessible > 0 {
        accessible = 0;
        let mut to_be_removed = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if grid[y][x] == '@' {
                    let mut neighbour_count = 0;

                    if x > 0 && y > 0 && grid[y - 1][x - 1] == '@' {
                        neighbour_count += 1;
                    }
                    if y > 0 && grid[y - 1][x] == '@' {
                        neighbour_count += 1;
                    }
                    if x < grid[y].len() - 1 && y > 0 && grid[y - 1][x + 1] == '@' {
                        neighbour_count += 1;
                    }
                    if x > 0 && grid[y][x - 1] == '@' {
                        neighbour_count += 1;
                    }
                    if x < grid[y].len() - 1 && grid[y][x + 1] == '@' {
                        neighbour_count += 1;
                    }
                    if x > 0 && y < grid.len() - 1 && grid[y + 1][x - 1] == '@' {
                        neighbour_count += 1;
                    }
                    if y < grid.len() - 1 && grid[y + 1][x] == '@' {
                        neighbour_count += 1;
                    }
                    if x < grid[y].len() - 1 && y < grid.len() - 1 && grid[y + 1][x + 1] == '@' {
                        neighbour_count += 1;
                    }

                    if neighbour_count < 4 {
                        accessible += 1;
                        to_be_removed.push((y, x));
                    }
                }
            }
        }

        for (y, x) in to_be_removed {
            grid[y][x] = '.';
        }

        removed += accessible;
    }

    removed
}

#[cfg(test)]
mod tests {
    use crate::{
        day04::{part1, part2},
        utils::read_example,
    };

    #[test]
    fn test_1() {
        let input = read_example(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_2() {
        let input = read_example(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(input), 43);
    }
}
