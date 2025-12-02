use crate::{
    aoc::{aoc_day::AocDayData, random_banner::banner},
    days::day2::day2,
};

mod aoc;
mod days;

fn main() {
    banner();

    let aocs: Vec<AocDayData> = vec![day2()];

    for aoc in aocs {
        aoc.solve();
    }
}
