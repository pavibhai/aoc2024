use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn part1(n: &Network) -> u32 {
    let sets = n.all_inter_connected(3);
    sets.iter()
        .filter(|s| s.iter().any(|v| v.starts_with('t')))
        .count() as u32
}

pub fn part2(n: &Network) -> String {
    n.max_possible_group().join(",")
}

pub fn generator(input: &str) -> Network {
    Network::new(input)
}

pub struct Network {
    connections: HashMap<String, Vec<String>>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        input.lines().for_each(|l| {
            let (c1, c2) = l.split_once('-').unwrap();
            connections.entry(c1.to_string())
                .and_modify(|v| v.push(c2.to_string()))
                .or_insert(vec![c2.to_string()]);
            connections.entry(c2.to_string())
                .and_modify(|v| v.push(c1.to_string()))
                .or_insert(vec![c1.to_string()]);
        });

        println!("Max connections: {}", connections.values().map(|v| v.len()).max().unwrap());

        Network { connections }
    }

    fn all_inter_connected(&self, n: usize) -> HashSet<Vec<String>> {
        let mut sets = HashMap::new();
        for (c, connections) in &self.connections {
            for combination in connections.iter().combinations(n - 1) {
                let mut s: Vec<String> = combination.iter()
                    .map(|&v| v.to_string()).collect();
                s.push(c.to_string());
                s.sort();
                sets.entry(s)
                    .and_modify(|v| *v += 1)
                    .or_insert(1u32);
            }
        }
        sets.drain().filter(|(_, v)| v == &(n as u32))
            .map(|(k, _)| k)
            .collect()
    }

    fn max_possible_group(&self) -> Vec<String> {
        let mut max_size = self.connections.iter()
            .map(|(_, v)| v.len() + 1)
            .max()
            .unwrap();

        while max_size > 2 {
            match self.all_inter_connected(max_size) {
                mut r if !r.is_empty() => {
                    return r.drain().next().unwrap();
                }
                _ => {}
            }
            max_size -= 1;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_generator() {
        let n = generator(&INPUT);
        assert_eq!(n.connections.get("kh").unwrap().len(), 4);
        assert_eq!(n.connections.get("qp").unwrap().len(), 4);
    }

    #[test]
    fn test_part_1() {
        let n = generator(&INPUT);
        assert_eq!(part1(&n), 7);
    }

    #[test]
    fn test_part_2() {
        let n = generator(&INPUT);
        assert_eq!(part2(&n), "co,de,ka,ta");
    }
}
