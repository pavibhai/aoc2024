use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn part1(n: &Network) -> u32 {
    let sets = n.all_inter_connected(3);
    sets.iter()
        .filter(|s| s.iter().any(|v| n.get_host(v).starts_with('t')))
        .count() as u32
}

pub fn part2(n: &Network) -> String {
    let mut r = n.max_possible_group().iter()
        .map(|v| n.get_host(v))
        .collect_vec();
    r.sort();
    r.join(",")
}

pub fn generator(input: &str) -> Network {
    Network::new(input)
}

pub struct Network {
    id_host_map: Vec<String>,
    _host_id_map: HashMap<String, u32>,
    connections: HashMap<u32, Vec<u32>>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut host_id_map: HashMap<String, u32> = HashMap::new();
        let mut id_host_map: Vec<String> = Vec::new();
        let mut connections: HashMap<u32, Vec<u32>> = HashMap::new();

        input.lines().for_each(|l| {
            let (c1, c2) = l.split_once('-').unwrap();
            let id1 = *host_id_map.entry(c1.to_string()).or_insert(id_host_map.len() as u32);
            if id_host_map.len() < host_id_map.len() { id_host_map.push(c1.to_owned()); }
            let id2 = *host_id_map.entry(c2.to_string()).or_insert(id_host_map.len() as u32);
            if id_host_map.len() < host_id_map.len() { id_host_map.push(c2.to_owned()); }

            connections.entry(id1)
                .and_modify(|v| v.push(id2))
                .or_insert(vec![id2]);
            connections.entry(id2)
                .and_modify(|v| v.push(id1))
                .or_insert(vec![id1]);
        });

        Network { id_host_map, _host_id_map: host_id_map, connections }
    }

    fn _get_id(&self, host: &str) -> &u32 {
        self._host_id_map.get(host).unwrap()
    }

    fn get_host(&self, id: &u32) -> &str {
        self.id_host_map.get(*id as usize).unwrap()
    }

    fn all_inter_connected(&self, n: usize) -> HashSet<Vec<u32>> {
        let mut sets = HashMap::new();
        for (c, connections) in &self.connections {
            for combination in connections.iter().combinations(n - 1) {
                let mut s: Vec<u32> = combination.iter()
                    .map(|&v| *v).collect();
                s.push(*c);
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

    fn max_possible_group(&self) -> Vec<u32> {
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

        assert_eq!(n.connections.get(n._get_id("kh")).unwrap().len(), 4);
        assert_eq!(n.connections.get(n._get_id("qp")).unwrap().len(), 4);
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
