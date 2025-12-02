use crate::aoc::aoc_day::AocDayData;

pub fn day2() -> AocDayData {
    AocDayData::new(02, "resources/day02".to_string(), solve_a, solve_b)
}

//  123123123
//  123123123
//  123 123 123
//  12 31 23 12 3

//  12312312311
//  123 123 123 11
//  1231 2312 311
//  12 31 23 12 31 1
//  1 2 3 1 2 3 1 2 3 1 1

fn solve_b(input: &str) -> i64 {
    let entries = prep_input(input);

    entries.iter().fold(0_i64, |acc, part| {
        fn is_repeating(seq: &Vec<String>) -> bool {
            seq.windows(2).all(|w| w[0] == w[1])
        }

        let part_str = part.to_string();
        let len = part_str.len();

        for size in (1..=(len / 2)).filter(|s| len % s == 0) {
            let parts = part_str
                .chars()
                .collect::<Vec<char>>()
                .chunks(size)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();
            if is_repeating(&parts) {
                return acc + part;
            }
        }

        acc
    })
}

fn solve_a(input: &str) -> i64 {
    let entries = prep_input(input);

    entries.iter().fold(0_i64, |acc, part| {
        let part_str = part.to_string();
        if part_str.len() % 2 != 0 {
            acc
        } else {
            let (first, second) = part_str.split_at(part_str.len() / 2);

            acc + if first == second { *part } else { 0_i64 }
        }
    })
}

fn prep_input(input: &str) -> Vec<i64> {
    let entries = input
        .split(",")
        .map(|entry| {
            let split = entry.split("-").collect::<Vec<&str>>();
            (split[0], split[1])
        })
        .flat_map(|(start, end)| start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    entries
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 1227775554);
    }
}
