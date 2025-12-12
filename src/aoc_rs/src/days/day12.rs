use crate::aoc::aoc_day::AocDayData;

pub fn day12() -> AocDayData {
    AocDayData::new(12, "resources/day12".to_string(), solve_a, solve_b)
}

type Num = i64;

fn solve_b(input: &str) -> i64 {
    0
}

/// The trick is that you don't actually need to try to solve this problem
/// on the 2D grid. Simply counting the needed tiles and comparing to the grid
/// size is enough.
fn solve_a(input: &str) -> i64 {
    let shapes = construct_shapes();
    let mut valid_lines = 0;
    for line in input.split("\n").skip(SHAPES_STR.split("\n").count()) {
        let part = line.split(":").collect::<Vec<_>>();
        let grid_size = part[0]
            .split("x")
            .map(|dims| dims.parse::<i64>().unwrap())
            .reduce(|acc, next| acc * next)
            .unwrap();

        let shape_count = part[1]
            .trim()
            .split(" ")
            .map(|el| el.parse::<Num>().unwrap())
            .enumerate()
            .collect::<Vec<_>>();

        let shape_elements_count = shape_count.iter().fold(0_i64, |acc, (idx, count)| {
            acc + shapes[*idx].element_count * count
        });
        if shape_elements_count <= grid_size {
            valid_lines += 1;
        }
    }

    valid_lines
}

// region: of shame
struct Shape {
    index: usize,
    content: &'static str,
    element_count: Num,
}

impl Shape {
    fn from_str(index: usize, content: &'static str) -> Self {
        let element_count = content.chars().filter(|c| *c == '#').count() as Num;

        Self {
            content,
            index,
            element_count,
        }
    }
}

fn construct_shapes() -> Vec<Shape> {
    vec![
        Shape::from_str(
            0,
            "###
##.
##.",
        ),
        Shape::from_str(
            1,
            "###
##.
.##",
        ),
        Shape::from_str(
            2,
            ".##
###
##.",
        ),
        Shape::from_str(
            3,
            "##.
###
##.",
        ),
        Shape::from_str(
            4,
            "###
#..
###",
        ),
        Shape::from_str(
            5,
            "###
.#.
###",
        ),
    ]
}

const SHAPES_STR: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###
";

// endregion

#[cfg(test)]
mod tests {
    use super::solve_a;
    // use super::solve_b;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    // #[test]
    // fn test_b() {
    //     let result = solve_b(TEST_INPUT.into());
    //     assert_eq!(result, 25272);
    // }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 2);
    }
}
