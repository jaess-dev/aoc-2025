use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::aoc::aoc_day::AocDayData;

/// NOTE:
/// I didn't solve task b. I did not come up with a solution I liked.
/// So I skipped it.
pub fn day9() -> AocDayData {
    AocDayData::solve_a(09, "resources/day09".to_string(), solve_a)
}

type Num = i64;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn calc_area(&self, end: &Coord2D) -> Num {
        (self.x as Num - end.x as Num + 1).abs() * (self.y as Num - end.y as Num + 1).abs()
    }
}

type Loop<'a> = HashSet<&'a Coord2D>;

fn solve_b(input: &str) -> i64 {
    // 1. preprocess
    let red_tiles = preprocess(input);
    // 2. areas with corners
    let mut areas = HashMap::new();
    for start in red_tiles.iter() {
        for end in red_tiles.iter() {
            if areas.contains_key(&(end, start)) {
                continue;
            }

            areas.insert((start, end), start.calc_area(end));
        }
    }
    let mut areas = areas.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
    // 3. sort by area
    areas.sort_by(|(_, area1), (_, area2)| area1.cmp(area2));

    // 4. for area calculate the loop for both structures
    let mut loops: Vec<HashSet<&Coord2D>> = vec![];
    for ((start, end), area) in areas.into_iter().rev() {
        if !loops.iter().any(|l| l.contains(start)) {
            loops.push(loop_creator(&red_tiles, start));
        };

        if !loops.iter().any(|l| l.contains(end)) {
            loops.push(loop_creator(&red_tiles, end));
        };

        let l1 = loops.iter().find(|l| l.contains(start)).unwrap();
        let l2 = loops.iter().find(|l| l.contains(end)).unwrap();

        // not in the same loop
        if l1 != l2 {
            continue;
        }

        // 5. check if they are in the same loop, and whether the other two
        //    corners are also in that loop
        let (corner1, corner2) = (Coord2D::new(start.x, end.y), Coord2D::new(end.x, start.y));
        if l1.contains(&corner1) || l1.contains(&corner2) {
            return *area;
        }
        // let l1_is_in = is_in_loop(l1, &corner1);
        // let l2_is_in = is_in_loop(l1, &corner2);
        // if l1_is_in && l2_is_in {
        //     return *area;
        // }
    }

    panic!()
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn is_in_loop(l: &Loop, c: &Coord2D) -> bool {
    fn is_in_loop_in_direction(l: &Loop, c: &Coord2D, d: Direction) -> bool {
        let mut higher = false;
        let mut lower = false;
        for col in l
            .iter()
            .filter(|el| match d {
                Direction::East => el.x > c.x,
                Direction::West => el.x < c.x,
                Direction::North => el.y > c.y,
                Direction::South => el.y < c.y,
            })
            .map(|el| {
                (
                    match d {
                        Direction::East | Direction::West => el.x,
                        Direction::North | Direction::South => el.y,
                    },
                    el,
                )
            })
            .into_group_map()
            .values()
            .filter(|col| col.len() >= 2)
        {
            for el in col {
                match d {
                    Direction::East | Direction::West => {
                        if el.y == c.y {
                            return true;
                        }
                        if el.y < c.y {
                            lower = true;
                        }
                        if el.y > c.y {
                            higher = true;
                        }
                    }
                    Direction::North | Direction::South => {
                        if el.x == c.x {
                            return true;
                        }
                        if el.x < c.x {
                            lower = true;
                        }
                        if el.x > c.x {
                            higher = true;
                        }
                    }
                };
            }
        }

        higher && lower
    }

    let x_map = l.iter().map(|el| (el.x, el)).into_group_map();
    let y_map = l.iter().map(|el| (el.y, el)).into_group_map();

    match x_map.get(&c.x) {
        Some(col) => {
            if col.len() >= 2 {
                let mut greater = false;
                let mut lower = false;
                for el in col {
                    if el.y == c.y {
                        greater = true;
                        lower = true;
                    }
                    if el.y > c.y {
                        greater = true;
                    }
                    if el.y < c.y {
                        lower = true;
                    }
                }
                if greater && lower {
                    return true;
                }
            }
        }
        None => {}
    }

    match y_map.get(&c.y) {
        Some(col) => {
            if col.len() >= 2 {
                let mut greater = false;
                let mut lower = false;
                for el in col {
                    if el.x == c.x {
                        greater = true;
                        lower = true;
                    }
                    if el.x > c.x {
                        greater = true;
                    }
                    if el.x < c.x {
                        lower = true;
                    }
                }
                if greater && lower {
                    return true;
                }
            }
        }
        None => {}
    }

    is_in_loop_in_direction(l, c, Direction::North)
        && is_in_loop_in_direction(l, c, Direction::East)
        && is_in_loop_in_direction(l, c, Direction::South)
        && is_in_loop_in_direction(l, c, Direction::West)
}

fn loop_creator<'a>(red_tiles: &'a Vec<Coord2D>, start: &'a Coord2D) -> Loop<'a> {
    fn union_by<'a>(
        new_to_visit: HashSet<&'a Coord2D>,
        red_tiles: &'a Vec<Coord2D>,
        predicate: impl Fn(&&Coord2D) -> bool,
    ) -> HashSet<&'a Coord2D> {
        new_to_visit
            .union(&red_tiles.iter().filter(predicate).collect())
            .map(|c| *c)
            .collect()
    }
    // goto east until no tile found any more
    // goto north until no tile found any more
    // goto west until no tile found any more
    // goto south, repeat until your back at start
    let mut to_visit = HashSet::new();
    let mut loop_structure = Loop::new();

    to_visit.insert(start);
    loop {
        let el = to_visit.iter().next();
        let el = match el {
            Some(el) => *el,
            None => break,
        };

        loop_structure.insert(el);
        let mut new_to_visit = to_visit.clone();
        new_to_visit.remove(el);

        new_to_visit = union_by(new_to_visit, red_tiles, |c| c.x == el.x);
        new_to_visit = union_by(new_to_visit, red_tiles, |c| c.y == el.y);
        new_to_visit = new_to_visit
            .into_iter()
            .filter(|el| !loop_structure.contains(el))
            .collect();

        to_visit = new_to_visit;
    }

    loop_structure
}

fn solve_a(input: &str) -> i64 {
    let red_tiles = preprocess(input);

    let mut areas = HashSet::new();
    for start in red_tiles.iter() {
        for end in red_tiles.iter() {
            areas.insert(start.calc_area(end));
        }
    }

    *areas.iter().max().unwrap()
}

fn preprocess(input: &str) -> Vec<Coord2D> {
    let red_tiles = input
        .split("\n")
        .map(|line| {
            let coord = line.split(",").collect::<Vec<&str>>();
            Coord2D::new(coord[0].parse().unwrap(), coord[1].parse().unwrap())
        })
        .collect::<Vec<Coord2D>>();
    red_tiles
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 24);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 50);
    }
}
