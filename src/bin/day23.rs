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
                        let triplet = vec![a, b, c]
                            .iter()
                            .sorted()
                            .map(|x| (*x).to_owned())
                            .collect();
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
}

fn main() {
    let fname = args().nth(1).unwrap();
    let network = Network::parse(&fname);
    println!("Part 1: {}", network.part1());
}
