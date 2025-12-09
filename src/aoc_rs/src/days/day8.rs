use std::collections::HashSet;

use crate::aoc::aoc_day::AocDayData;

pub fn day8() -> AocDayData {
    AocDayData::new(08, "resources/day08".to_string(), solve_a, solve_b)
}

type Num = i64;
type Pos = (usize, usize, usize);
type Circuit = HashSet<Pos>;

const LIMIT: usize = 1_000;

fn solve_b(input: &str) -> i64 {
    let positions = input
        .split("\n")
        .map(|line| {
            let vals = line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (vals[0], vals[1], vals[2])
        })
        .collect::<Vec<Pos>>();

    // calculate all n * n distances
    // never store (a, b) and (b, a)
    // store them Vec<(Pos, Pos, distance)> -> sort by distance
    let mut pos_with_dist: Vec<(Pos, Pos, Num)> = vec![];
    {
        let mut added = HashSet::new();
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                if added.contains(&(pos2, pos1)) {
                    continue;
                }

                let d = dist(pos1, pos2);
                pos_with_dist.push((pos1, pos2, d));
                added.insert((pos1, pos2));
            }
        }
    }
    pos_with_dist.sort_by(|el1, el2| el1.2.cmp(&el2.2));

    // build circuit structure
    // type Circuit = Set<pos>
    // -> Set<Circuit>
    // -> Map<Pos, &Circuit>
    let mut own = Vec::<Circuit>::new();
    for el in positions {
        let mut c = Circuit::new();
        c.insert(el);
        own.push(c);
    }

    // iter over sorted distances (pos1, pos2, _)
    // -> let c1 = map[pos1]
    // -> let c2 = map[pos2]
    // remove from Set<Circuit> c1, c2
    // join c1, c2 and into cj
    // add cj to Set<Circuit>
    // map[pos1] = &cj
    // map[pos2] = &cj

    for (pos1, pos2, _) in pos_with_dist.iter() {
        fn get_c_with_pos<'a>(
            own: &'a Vec<Circuit>,
            pos: &'a Pos,
        ) -> Option<(usize, &'a HashSet<(usize, usize, usize)>)> {
            own.iter().enumerate().find(|(_, el)| el.contains(pos))
        }

        let (idx1, _) = get_c_with_pos(&own, pos1).unwrap();
        let c1 = own.remove(idx1);

        let (idx2, _) = match get_c_with_pos(&own, pos2) {
            None => {
                own.push(c1);
                continue;
            }
            Some(s) => s,
        };

        let c2 = own.remove(idx2);

        let union = c1.union(&c2).map(|el| *el).collect::<Circuit>();
        own.push(union);

        if own.len() == 1 {
            return pos1.0 as Num * pos2.0 as Num;
        }
    }

    panic!();
}

/// calculate all n * n distances
/// never store (a, b) and (b, a)
/// store them Vec<(Pos, Pos, distance)> -> sort by distance
///
/// build circuit structure
/// type Circuit = Set<pos>
/// -> Set<Circuit>
/// -> Map<Pos, &Circuit>
///
/// iter over sorted distances (pos1, pos2, _)
/// -> let c1 = map[pos1]
/// -> let c2 = map[pos2]
/// remove from Set<Circuit> c1, c2
/// join c1, c2 and into cj
/// add cj to Set<Circuit>
/// map[pos1] = &cj
/// map[pos2] = &cj
///
/// finisher:
/// get 3 largest circuits from Set<Circuit> -> multiply lengths
fn solve_a(input: &str) -> i64 {
    solve_a_for_test(input, LIMIT)
}

fn solve_a_for_test(input: &str, limit: usize) -> i64 {
    let positions = input
        .split("\n")
        .map(|line| {
            let vals = line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (vals[0], vals[1], vals[2])
        })
        .collect::<Vec<Pos>>();

    // calculate all n * n distances
    // never store (a, b) and (b, a)
    // store them Vec<(Pos, Pos, distance)> -> sort by distance
    let mut pos_with_dist: Vec<(Pos, Pos, Num)> = vec![];
    {
        let mut added = HashSet::new();
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                if added.contains(&(pos2, pos1)) {
                    continue;
                }

                let d = dist(pos1, pos2);
                pos_with_dist.push((pos1, pos2, d));
                added.insert((pos1, pos2));
            }
        }
    }
    pos_with_dist.sort_by(|el1, el2| el1.2.cmp(&el2.2));

    // build circuit structure
    // type Circuit = Set<pos>
    // -> Set<Circuit>
    // -> Map<Pos, &Circuit>
    let mut own = Vec::<Circuit>::new();
    for el in positions {
        let mut c = Circuit::new();
        c.insert(el);
        own.push(c);
    }

    // iter over sorted distances (pos1, pos2, _)
    // -> let c1 = map[pos1]
    // -> let c2 = map[pos2]
    // remove from Set<Circuit> c1, c2
    // join c1, c2 and into cj
    // add cj to Set<Circuit>
    // map[pos1] = &cj
    // map[pos2] = &cj

    for (pos1, pos2, _) in pos_with_dist.iter().take(limit) {
        fn get_c_with_pos<'a>(
            own: &'a Vec<Circuit>,
            pos: &'a Pos,
        ) -> Option<(usize, &'a HashSet<(usize, usize, usize)>)> {
            own.iter().enumerate().find(|(_, el)| el.contains(pos))
        }

        let (idx1, _) = get_c_with_pos(&own, pos1).unwrap();
        let c1 = own.remove(idx1);

        let (idx2, _) = match get_c_with_pos(&own, pos2) {
            None => {
                own.push(c1);
                continue;
            }
            Some(s) => s,
        };

        let c2 = own.remove(idx2);

        let union = c1.union(&c2).map(|el| *el).collect::<Circuit>();
        own.push(union);
    }

    // finisher:
    // get 3 largest circuits from Set<Circuit> -> multiply lengths
    let mut c_lengths: Vec<usize> = own.iter().map(|c| c.len()).collect();
    c_lengths.sort();

    c_lengths
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, &next| acc * (next as i64))
}

fn dist(pos1: Pos, pos2: Pos) -> Num {
    fn term(x1: usize, x2: usize) -> Num {
        (x1 as Num - x2 as Num).pow(2)
    }

    term(pos1.0, pos2.0) + term(pos1.1, pos2.1) + term(pos1.2, pos2.2)
}

#[cfg(test)]
mod tests {
    use super::solve_a_for_test;
    use super::solve_b;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT.into());
        assert_eq!(result, 25272);
    }

    #[test]
    fn test_a() {
        let result = solve_a_for_test(TEST_INPUT.into(), 10);
        assert_eq!(result, 40);
    }
}
