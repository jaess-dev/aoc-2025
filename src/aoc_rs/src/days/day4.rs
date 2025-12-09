use crate::aoc::aoc_day::AocDayData;

pub fn day4() -> AocDayData {
    AocDayData::new(04, "resources/day04".to_string(), solve_a, solve_b)
}

fn solve_b(input: &str) -> i64 {
    let mut grid = create_grid(input);

    let mut count = 0_i64;
    loop {
        let accessible_positions = calculate_accessible_positions(&grid);
        count += accessible_positions.iter().count() as i64;

        if accessible_positions.is_empty() {
            break;
        }

        for (i, j) in accessible_positions {
            grid[i][j] = ".";
        }
    }

    count
}

fn solve_a(input: &str) -> i64 {
    let grid = create_grid(input);

    let accessible_positions = calculate_accessible_positions(&grid);

    accessible_positions.iter().count() as i64
}

fn calculate_accessible_positions(grid: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut accessible_positions = Vec::<(usize, usize)>::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != "@" {
                continue;
            }

            let mut line1 = get_neighbors(&grid, i.checked_sub(1), j, false);
            let line2 = get_neighbors(&grid, Some(i), j, true);
            let line3 = get_neighbors(
                &grid,
                if i + 1 >= grid.len() {
                    None
                } else {
                    Some(i + 1)
                },
                j,
                false,
            );

            line1.extend_from_slice(&line2);
            line1.extend_from_slice(&line3);

            if line1.iter().filter(|&&&el| el == "@").count() < 4 {
                accessible_positions.push((i, j));
            }
        }
    }

    accessible_positions
}

fn get_neighbors<'a>(
    grid: &'a Vec<Vec<&'a str>>,
    i: Option<usize>,
    j: usize,
    is_middle: bool,
) -> Vec<&'a &'a str> {
    match i {
        None => vec![],
        Some(i) => {
            let res = grid
                .get(i)
                .iter()
                .flat_map(|&line| {
                    vec![
                        j.checked_sub(1).map_or(None, |j| line.get(j)),
                        line.get(j)
                            .map_or(None, |x| if is_middle { None } else { Some(x) }),
                        if j + 1 >= grid[0].len() {
                            None
                        } else {
                            Some(j + 1)
                        }
                        .map_or(None, |j| line.get(j)),
                    ]
                })
                .filter_map(|opt| opt)
                .collect();
            res
        }
    }
}

fn create_grid(input: &str) -> Vec<Vec<&str>> {
    let grid = input
        .split("\n")
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect::<Vec<Vec<&str>>>();
    grid
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 43);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 13);
    }
}
