use std::collections::{HashMap, HashSet};

use crate::aoc::aoc_day::AocDayData;

pub fn day11() -> AocDayData {
    AocDayData::new(11, "resources/day11".to_string(), solve_a, solve_b)
}

fn solve_a(input: &str) -> Num {
    let table = input
        .split("\n")
        .fold(HashMap::<&str, Vec<&str>>::new(), |mut acc, line| {
            let parts = line.split(":").collect::<Vec<_>>();
            let key = parts[0];
            let values = parts[1].trim().split(" ").collect();

            acc.insert(key, values);
            acc
        });

    let mut node_list = vec![vec!["you"]];
    loop {
        let (next_node_list, nodes_to_iter): (Vec<Vec<&str>>, Vec<Vec<&str>>) = node_list
            .into_iter()
            .partition(|l| *l.last().unwrap() == "out");

        node_list = next_node_list;
        if nodes_to_iter.is_empty() {
            break;
        }

        for node in nodes_to_iter {
            for &next in table[node.last().unwrap()].iter() {
                let mut node_clone = node.clone();
                node_clone.push(next);
                node_list.push(node_clone);
            }
        }
    }

    node_list.iter().count() as Num
}

struct Tree {
    root: Node,
}

struct Node {
    children: Vec<Node>,
}

type Num = i64;

fn solve_b(input: &str) -> Num {
    let mut table =
        input
            .split("\n")
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut acc, line| {
                let parts = line.split(":").collect::<Vec<_>>();
                let key = parts[0];
                let values = parts[1].trim().split(" ").collect();

                acc.insert(key, values);
                acc
            });

    // path reduction removing 1:1 mappings
    // FIXME: doesn't work, can't figure out why
    // path_reduction(&mut table);

    // build list with nodes that can never lead to dac or fft

    // 1. build reverse table and starting nodes are "dac" and "fft", end node is "svr"
    let reversed_table: HashMap<&str, HashSet<&str>> = build_reverse_table(&table);

    // 2. find the paths similar to already constructed algorithm
    let mut starting_points =
        find_way_to_end(&reversed_table, vec![vec!["dac"], vec!["fft"]], "svr");

    // 3. reduce paths to not have duplicates
    starting_points = starting_points
        .into_iter()
        .filter(|hs| hs.contains(&"dac") && hs.contains(&"fft"))
        .collect();
    for col in starting_points.iter_mut() {
        col.reverse();
    }

    // 4. run algorithm into the normal direction
    let mut hs = HashMap::new();
    let dac_count = starting_points
        .iter()
        .filter(|points| *points.last().unwrap() == "dac")
        .count() as i64;
    let fft_count = starting_points
        .iter()
        .filter(|points| *points.last().unwrap() == "fft")
        .count() as i64;

    if dac_count > 0 {
        hs.insert("dac", dac_count);
    }
    if fft_count > 0 {
        hs.insert("fft", fft_count);
    }
    let node_list = find_way_to_end_reducing(&table, hs, "out");

    // 5. count
    *node_list.get("out").unwrap()
}

fn build_reverse_table<'a>(
    table: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut revered_table: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (k, vals) in table.iter() {
        for val in vals {
            match revered_table.get_mut(val) {
                None => {
                    let mut rhs = HashSet::new();
                    rhs.insert(*k);
                    revered_table.insert(val, rhs);
                }
                Some(rhs) => {
                    rhs.insert(*k);
                }
            }
        }
    }

    revered_table
}

fn find_way_to_end<'a>(
    table: &HashMap<&'a str, HashSet<&'a str>>,
    mut node_list: Vec<Vec<&'a str>>,
    end_node: &'a str,
) -> Vec<Vec<&'a str>> {
    loop {
        let (next_node_list, nodes_to_iter): (Vec<Vec<&str>>, Vec<Vec<&str>>) = node_list
            .into_iter()
            .partition(|l| *l.last().unwrap() == end_node);

        node_list = next_node_list;
        if nodes_to_iter.is_empty() {
            break;
        }

        for node in nodes_to_iter {
            for &next in table[node.last().unwrap()].iter() {
                let mut node_clone = node.clone();
                node_clone.push(next);
                node_list.push(node_clone);
            }
        }
    }
    node_list
}

fn find_way_to_end_reducing<'a>(
    table: &HashMap<&'a str, HashSet<&'a str>>,
    mut node_list: HashMap<&'a str, Num>,
    end_node: &'a str,
) -> HashMap<&'a str, Num> {
    loop {
        let (next_node_list, nodes_to_iter) = node_list.into_iter().partition(|l| l.0 == end_node);

        node_list = next_node_list;
        if nodes_to_iter.is_empty() {
            break;
        }

        for node in nodes_to_iter {
            for &next in table[node.0].iter() {
                match node_list.get(next) {
                    None => {
                        node_list.insert(next, node.1);
                    }
                    Some(counter) => {
                        node_list.insert(next, node.1 + *counter);
                    }
                }
            }
        }
    }

    node_list
}

#[cfg(test)]
mod tests {
    use super::solve_a;
    use super::solve_b;

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_b() {
        let result = solve_b(TEST_INPUT_B.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_a() {
        let result = solve_a(TEST_INPUT.into());
        assert_eq!(result, 5);
    }
}
