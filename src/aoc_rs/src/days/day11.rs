use std::{
    collections::{HashMap, HashSet},
    default,
};

use crate::aoc::aoc_day::AocDayData;

pub fn day11() -> AocDayData {
    AocDayData::new(11, "resources/day11".to_string(), solve_a, solve_b)
}

#[derive(Default)]
struct Graph<'a> {
    nodes: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    pub fn add(&mut self, src: &'a str, dest: &'a str) {
        if !self.nodes.contains_key(src) {
            self.nodes.insert(src, vec![]);
        }

        self.nodes.get_mut(src).unwrap().push(dest);
    }

    pub fn count_paths(&self, start: &str, dest: &str) -> Num {
        self.count_paths_excluding(start, dest, HashSet::new())
    }

    pub fn count_paths_excluding(&self, start: &str, dest: &str, exclude: HashSet<&str>) -> Num {
        let mut current: HashMap<&str, Num> = self
            .nodes
            .get(start)
            .expect(&format!("Called unwrap on {}", start))
            .iter()
            .map(|s| (*s, 1))
            .collect();

        let mut count = 0;
        while !current.is_empty() {
            let mut new_current = HashMap::new();
            for (cur, cur_count) in current {
                if exclude.contains(cur) {
                    continue;
                }
                if cur == dest {
                    count += cur_count;
                    continue;
                }
                if cur == "out" {
                    continue;
                }

                for &next in self.nodes.get(cur).unwrap() {
                    new_current.insert(next, new_current.get(next).unwrap_or(&0) + cur_count);
                }
            }

            current = new_current;
        }

        count
    }
}

type Num = i64;

fn solve_b(input: &str) -> Num {
    let mut graph = input.split("\n").fold(Graph::default(), |mut acc, line| {
        let parts = line.split(":").collect::<Vec<_>>();
        let origin = parts[0];
        let destination: Vec<&str> = parts[1].trim().split(" ").collect();

        for dest in destination {
            acc.add(origin, dest);
        }

        acc
    });

    // construct graph
    // 1. find all ways from svr to fft and dac
    // 2. find all ways from fft to dac and from dac to fft
    // 3. find all ways from fft to out and all from dac to out
    // 4. (svr -> fft) * (fft -> dac) * (dac -> out)
    //    (svr -> dac) * (dac -> fft) * (fft -> out)

    (graph.count_paths_excluding("svr", "fft", {
        let mut set = HashSet::new();
        set.insert("dac");
        set
    }) * graph.count_paths("fft", "dac")
        * graph.count_paths_excluding("dac", "out", {
            let mut set = HashSet::new();
            set.insert("fft");
            set
        }))
        + graph.count_paths_excluding("svr", "dac", {
            let mut set = HashSet::new();
            set.insert("fft");
            set
        }) * graph.count_paths("dac", "fft")
            * graph.count_paths_excluding("fft", "out", {
                let mut set = HashSet::new();
                set.insert("dac");
                set
            })
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
    use super::*;

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
