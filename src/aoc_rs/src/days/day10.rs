#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    mem::take,
    os::linux::raw::stat,
};

use crate::aoc::aoc_day::AocDayData;

pub fn day10() -> AocDayData {
    AocDayData::new(10, "resources/day10".to_string(), solve_a, solve_b)
}

type Num = i64;

fn solve_b(input: &str) -> i64 {
    let machines: Vec<_> = input.split("\n").map(Machine::from_line).collect();

    let mut tototal_mutations = 0;
    for machine in machines {
        let mut mutations = 0;

        let mut states = HashSet::new();
        states.insert(
            machine
                .joltages
                .iter()
                .map(|_| 0)
                .collect::<Vec<_>>()
                .clone(),
        );

        while states.iter().all(|l| *l != machine.joltages) {
            //
            let mut new_states = states.clone();
            for state in states.iter() {
                for button in machine.buttons.iter() {
                    let mut state = state.clone();
                    for &pos in button.iter() {
                        state[pos] += 1;
                    }
                    new_states.insert(state);
                }
            }

            states = new_states
                .into_iter()
                .filter(|state| state.iter().zip(&machine.joltages).all(|(x, y)| x <= y))
                .collect();

            mutations += 1;
        }

        tototal_mutations += mutations;
    }

    tototal_mutations
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<Num>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let contents: Vec<_> = line.split(" ").collect();
        let lights = contents
            .first()
            .unwrap()
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!(),
            })
            .collect();

        let buttons = contents
            .iter()
            .skip(1)
            .take(contents.len() - 2)
            .map(|button| {
                button
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|pos| pos.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let joltages = contents
            .last()
            .unwrap()
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .map(|d| d.parse::<Num>().unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

fn solve_a(input: &str) -> i64 {
    let machines: Vec<_> = input.split("\n").map(Machine::from_line).collect();

    let mut tototal_mutations = 0;
    for machine in machines {
        let mut mutations = 0;

        let mut states = HashSet::new();
        states.insert(
            machine
                .lights
                .iter()
                .map(|_| false)
                .collect::<Vec<_>>()
                .clone(),
        );

        while states.iter().all(|l| *l != machine.lights) {
            //
            let mut new_states = states.clone();
            for state in states.iter() {
                for button in machine.buttons.iter() {
                    let mut state = state.clone();
                    for &pos in button.iter() {
                        state[pos] = !state[pos];
                    }
                    new_states.insert(state);
                }
            }

            states = new_states;
            mutations += 1;
        }

        tototal_mutations += mutations;
    }

    tototal_mutations
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 33);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 7);
    }
}
