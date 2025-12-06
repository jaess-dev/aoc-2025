use crate::aoc::aoc_day::AocDayData;

pub fn day6() -> AocDayData {
    AocDayData::new(06, "resources/day06".to_string(), solve_a, solve_b)
}

type Num = i64;

fn solve_b(input: &str) -> i64 {
    let mut rows = input
        .split("\n")
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let operator_row = rows.pop().unwrap();

    let mut total = 0;

    let mut start = 0;
    let mut end: usize = 1;

    while end < operator_row.len() {
        loop {
            match operator_row.get(end).map(|&c| c == ' ') {
                Some(b) if !b => {
                    break;
                }
                None => {
                    end += 1;
                    break;
                }
                _ => {
                    end += 1;
                }
            }
        }

        let mut nums = vec![];
        for i in start..end - 1 {
            let mut numv = vec![];
            for row in rows.iter() {
                match row.get(i).map_or(None, |c| c.to_digit(10)) {
                    Some(num) => numv.push(num as i64),
                    None => {}
                }
            }
            let len = numv.len();

            let num = numv.iter().enumerate().fold(0 as Num, |acc, (idx, next)| {
                acc + (10_i64.pow((len - idx - 1) as u32) * next) as Num
            });
            nums.push(num);
        }

        total += calculate_term(&operator_row.get(start).unwrap().to_string(), &nums);

        start = end;
        end = end + 1;
    }

    total
}

fn solve_a(input: &str) -> i64 {
    let (operations, grid) = parse_base(input);

    let mut num_rows: Vec<Vec<i64>> = grid.first().unwrap().iter().map(|_| vec![]).collect();
    for next in grid.into_iter() {
        for (j, &el) in next.iter().enumerate() {
            num_rows
                .get_mut(j)
                .unwrap()
                .push(el.parse::<Num>().unwrap());
        }
    }

    calculate(operations, num_rows)
}

fn calculate(operations: Vec<&str>, num_rows: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for (&op, numbers) in operations.iter().zip(num_rows) {
        let res = calculate_term(op, &numbers);

        total += res;
    }

    total
}

fn calculate_term(op: &str, numbers: &Vec<i64>) -> i64 {
    let (mut res, op) = match op {
        "*" => (1_i64, i64::unchecked_mul as unsafe fn(i64, i64) -> i64),
        "+" => (0_i64, i64::unchecked_add as unsafe fn(i64, i64) -> i64),
        _ => {
            panic!()
        }
    };

    for &num in numbers.iter() {
        res = unsafe { op(res, num) };
    }
    res
}

fn parse_base(input: &str) -> (Vec<&str>, Vec<Vec<&str>>) {
    let mut grid: Vec<Vec<&str>> = input
        .split("\n")
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operations = grid.pop().unwrap();

    (operations, grid)
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 3_263_827);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 4_277_556);
    }
}
