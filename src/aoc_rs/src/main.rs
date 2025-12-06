use crate::{
    aoc::{aoc_day::AocDayData, random_banner::banner},
    days::{day2::day2, day3::day3, day4::day4, day5::day5, day6::day6},
};

mod aoc;
mod days;

fn main() {
    banner();

    let aocs: Vec<AocDayData> = vec![day6(), day5(), day4(), day3(), day2()];

    for aoc in aocs {
        aoc.solve();
    }
}
