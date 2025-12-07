use std::collections::{HashMap, HashSet};

use crate::aoc::aoc_day::AocDayData;

pub fn day7() -> AocDayData {
    AocDayData::new(07, "resources/day07".to_string(), solve_a, solve_b)
}

type Num = i64;
type Pos = (usize, usize);

trait Upserter {
    fn upsert(&mut self, pos: Pos, count: Num);
}

impl Upserter for HashMap<Pos, Num> {
    fn upsert(&mut self, pos: Pos, count: Num) {
        self.insert(
            pos,
            match self.get(&pos) {
                Some(old_count) => old_count + count,
                None => count,
            },
        );
    }
}

fn solve_b(input: &str) -> i64 {
    let (grid, idx) = grid_with_start(input);

    let mut starting_positions = HashMap::new();
    starting_positions.insert((0_usize, idx), 1);

    for _ in 0..grid.len() - 1 {
        let mut next_start_pos = HashMap::new();
        for ((x, y), count) in starting_positions {
            let x = x + 1;

            if grid[x][y] == '^' {
                next_start_pos.upsert((x, y - 1), count);
                next_start_pos.upsert((x, y + 1), count);
            } else {
                next_start_pos.upsert((x, y), count);
            }
        }
        starting_positions = next_start_pos;
    }

    starting_positions.values().sum()
}

fn solve_a(input: &str) -> i64 {
    let (grid, idx) = grid_with_start(input);

    let mut starting_positions = HashSet::new();
    starting_positions.insert((0_usize, idx));

    let mut split_count = 0;
    for i in 0..grid.len() - 1 {
        let mut next_start_pos = HashSet::new();
        for (x, y) in starting_positions {
            let x = x + 1;
            if grid[x][y] == '^' {
                next_start_pos.insert((x, y - 1));
                next_start_pos.insert((x, y + 1));
                split_count += 1;
            } else {
                next_start_pos.insert((x, y));
            }
        }
        starting_positions = next_start_pos;
    }
    split_count
}

fn grid_with_start(input: &str) -> (Vec<Vec<char>>, usize) {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let (idx, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(_, el)| **el == 'S')
        .unwrap();
    (grid, idx)
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 40);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 21);
    }
}
