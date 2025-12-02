use crate::{aoc::aoc_day::AocDayData, days::day2::day2};

mod aoc;
mod days;

fn main() {
    let aocs: Vec<AocDayData> = vec![day2()];

    for aoc in aocs {
        aoc.solve();
    }
}
