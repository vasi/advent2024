use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

type Computer = String;

#[derive(Debug, Clone)]
struct Network {
    connections: HashMap<Computer, HashSet<Computer>>,
}

impl Network {
    fn parse(fname: &str) -> Self {
        let mut connections: HashMap<Computer, HashSet<Computer>> = HashMap::new();
        let contents = read_to_string(fname).unwrap();
        for line in contents.lines() {
            let (a, b) = line.split_once('-').unwrap();
            connections
                .entry(a.to_owned())
                .or_default()
                .insert(b.to_owned());
            connections
                .entry(b.to_owned())
                .or_default()
                .insert(a.to_owned());
        }
        Self { connections }
    }

    fn computers(&self) -> impl Iterator<Item = &String> {
        self.connections.keys()
    }

    fn triplets(&self) -> HashSet<Vec<Computer>> {
        let mut ret = HashSet::new();
        for a in self.computers() {
            let aconn = self.connections.get(a).unwrap();
            for b in aconn {
                if a != b {
                    let bconn = self.connections.get(b).unwrap();
                    for c in aconn.intersection(bconn) {
                        let triplet = vec![a, b, c].iter().cloned().cloned().sorted().collect();
                        ret.insert(triplet);
                    }
                }
            }
        }
        ret
    }

    fn part1(&self) -> usize {
        self.triplets()
            .iter()
            .filter(|tr| tr.iter().any(|s| s.starts_with("t")))
            .count()
    }

    fn bk_simple(
        &self,
        r: HashSet<Computer>,
        mut p: HashSet<Computer>,
        mut x: HashSet<Computer>,
        m: &mut Vec<HashSet<Computer>>,
    ) {
        if p.is_empty() && x.is_empty() {
            m.push(r.clone());
        }
        for v in p.clone() {
            let nv = self.connections.get(&v).unwrap();
            self.bk_simple(
                r.iter().chain(Some(&v)).cloned().collect(),
                p.intersection(nv).cloned().collect(),
                x.intersection(nv).cloned().collect(),
                m,
            );
            p.remove(&v);
            x.insert(v);
        }
    }

    fn maximal_cliques(&self) -> Vec<HashSet<Computer>> {
        let mut m = Vec::new();
        self.bk_simple(
            HashSet::new(),
            self.connections.keys().map(|s| s.to_owned()).collect(),
            HashSet::new(),
            &mut m,
        );
        m
    }

    fn part2(&self) -> String {
        let cliques = self.maximal_cliques();
        let max = cliques.iter().max_by_key(|c| c.len()).unwrap();
        max.iter().sorted().join(",")
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let network = Network::parse(&fname);
    println!("Part 1: {}", network.part1());
    println!("Part 2: {}", network.part2());
}
