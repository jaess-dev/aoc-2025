#![allow(dead_code)]
use std::fs;

pub trait AocDay {
    fn get_day_number(&self) -> u8;
    fn get_file_path(&self) -> String;
    fn solve_a(&self, input: &str) -> Option<i64>;
    fn solve_b(&self, input: &str) -> Option<i64>;
}

pub struct AocDayData {
    day: u8,
    file: String,
    solve_a_fn: Option<fn(&str) -> i64>,
    solve_b_fn: Option<fn(&str) -> i64>,
}

impl AocDayData {
    pub fn empty(day: u8) -> Self {
        Self {
            day,
            file: "".to_string(),
            solve_a_fn: None,
            solve_b_fn: None,
        }
    }
    pub const fn solve_a(day: u8, file: String, solve_a_fn: fn(&str) -> i64) -> Self {
        Self {
            day,
            file,
            solve_a_fn: Some(solve_a_fn),
            solve_b_fn: None,
        }
    }

    pub const fn new(
        day: u8,
        file: String,
        solve_a_fn: fn(&str) -> i64,
        solve_b_fn: fn(&str) -> i64,
    ) -> Self {
        Self {
            day,
            file,
            solve_a_fn: Some(solve_a_fn),
            solve_b_fn: Some(solve_b_fn),
        }
    }

    pub fn solve(&self) {
        let day_number = self.get_day_number();
        let day_file = self.get_file_path();
        let content = fs::read_to_string(&day_file).expect("Could not find input file!");

        if let Some(res) = self.solve_a(&content) {
            println!("Result of Day {day_number} part a is {res}");
        }

        if let Some(res) = self.solve_b(&content) {
            println!("Result of Day {day_number} part b is {res}");
        }
    }
}

impl AocDay for AocDayData {
    fn get_day_number(&self) -> u8 {
        self.day
    }

    fn get_file_path(&self) -> String {
        self.file.to_string()
    }

    fn solve_a(&self, input: &str) -> Option<i64> {
        match self.solve_a_fn {
            None => {
                println!("No Solution for a was given!");
                None
            }
            Some(solve) => Some(solve(input)),
        }
    }

    fn solve_b(&self, input: &str) -> Option<i64> {
        match self.solve_b_fn {
            None => {
                println!("No Solution for b was given!");
                None
            }
            Some(solve) => Some(solve(input)),
        }
    }
}
