use std::collections::HashSet;

use crate::aoc::aoc_day::AocDayData;

pub fn day5() -> AocDayData {
    AocDayData::new(05, "resources/day05".to_string(), solve_a, solve_b)
}

fn solve_b(input: &str) -> i64 {
    let (_, ranges) = ingredients_extraction(input);
    ranges.iter().flat_map(|r| r.to_range()).count() as i64
}

type Num = u64;

struct Range {
    start: Num,
    end: Num,
}

impl Range {
    fn new(start: Num, end: Num) -> Self {
        Self { start, end }
    }

    fn to_range(&self) -> std::ops::RangeInclusive<u64> {
        self.start..=self.end
    }

    fn join(&self, other: &Self) -> Option<Range> {
        // no intersection if: r1 < l2 or r2 < l1.
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Self::new(
                Num::min(self.start, other.start),
                Num::max(self.end, other.end),
            ))
        }
    }
}

impl Containable<Num> for Range {
    fn contains(&self, num: &Num) -> bool {
        self.start <= *num && *num <= self.end
    }
}

trait Containable<T> {
    fn contains(&self, t: &T) -> bool;
}

impl Containable<Num> for Vec<Range> {
    fn contains(&self, t: &Num) -> bool {
        self.iter().any(|range| range.contains(t))
    }
}

fn solve_a(input: &str) -> i64 {
    let (ingredients, fresh_ingredients) = ingredients_extraction(input);

    ingredients
        .split("\n")
        .map(|line| line.parse::<Num>().unwrap())
        .filter(|i| fresh_ingredients.contains(i))
        .collect::<HashSet<u64>>()
        .len() as i64
}

fn ingredients_extraction(input: &str) -> (&str, Vec<Range>) {
    let ranges_and_ingredients = input.split("\n\n").collect::<Vec<&str>>();
    let ranges = *ranges_and_ingredients.first().unwrap();
    let ingredients = *ranges_and_ingredients.last().unwrap();

    let fresh_ingredients = ranges
        .split("\n")
        .map(|line| {
            let split = line.split("-").collect::<Vec<&str>>();
            let start = split.first().unwrap().parse::<Num>().unwrap();
            let end = split.last().unwrap().parse::<Num>().unwrap();

            Range::new(start, end)
        })
        .fold(Vec::<Range>::new(), |mut acc, next| {
            let mut current = next;

            loop {
                let mut new_acc = vec![];
                let mut changed = false;

                for el in acc {
                    match current.join(&el) {
                        Some(joined) => {
                            current = joined;
                            changed = true;
                        }
                        None => new_acc.push(el),
                    }
                }

                acc = new_acc;
                if !changed {
                    break;
                }
            }

            acc.push(current);
            acc
        });
    (ingredients, fresh_ingredients)
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 14);
    }

    #[test]
    fn test_b_possible() {
        let result = solve_b(TEST_INPUT.into());
        assert!(result < 342018167474526);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 3);
    }
}
