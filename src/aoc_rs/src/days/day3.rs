use crate::aoc::aoc_day::AocDayData;

pub fn day3() -> AocDayData {
    AocDayData::new(03, "resources/day03".to_string(), solve_a, solve_b)
}

fn solve_b(input: &str) -> i64 {
    let banks: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let result: Vec<i64> = banks
        .iter()
        .map(|bank| {
            let mut result = 0_i64;
            let mut start_idx = 0;
            for i in (0..=11).rev() {
                let col = bank[start_idx..bank.len() - i]
                    .iter()
                    .map(|val| val.clone())
                    .collect();
                let (tmp_start_idx, val) = largest_digit_with_idx(&col);
                start_idx += tmp_start_idx + 1;

                let intermediate = val as i64 * (10_i64.pow(i as u32));
                result += intermediate;
            }

            result
        })
        .collect();

    result.into_iter().sum::<i64>()
}

fn solve_a(input: &str) -> i64 {
    let banks: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let result: Vec<i64> = banks
        .iter()
        .map(|bank| {
            let (start_idx, start) = largest_digit_with_idx(
                &bank
                    .iter()
                    .map(|val| val.clone())
                    .take(bank.len() - 1)
                    .collect(),
            );

            let rest = bank
                .iter()
                .map(|val| val.clone())
                .skip(start_idx + 1)
                .collect();
            let (_, end) = largest_digit_with_idx(&rest);

            let val = (start * 10 + end) as i64;

            val
        })
        .collect();

    result.into_iter().sum::<i64>()
}

fn largest_digit_with_idx(v: &Vec<u32>) -> (usize, u32) {
    let val = v.iter().max().unwrap();
    v.iter()
        .enumerate()
        .map(|val| (val.0, val.1.clone()))
        .find(|t| t.1 == *val)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 357);
    }
}
