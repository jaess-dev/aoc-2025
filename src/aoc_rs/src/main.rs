use crate::{
    aoc::{aoc_day::AocDayData, random_banner::banner},
    days::{day2::day2, day3::day3, day4::day4},
};

mod aoc;
mod days;

fn main() {
    banner();

    let aocs: Vec<AocDayData> = vec![day4(), day3(), day2()];

    for aoc in aocs {
        aoc.solve();
    }
}
