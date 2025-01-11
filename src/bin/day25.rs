use itertools::Itertools;
use std::env::args;
use std::fs::read_to_string;

type Key = Vec<usize>;
type Lock = Vec<usize>;

#[derive(Debug)]
struct Schematics {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl Schematics {
    fn parse_item(lines: Vec<Vec<char>>) -> Vec<usize> {
        (0..5)
            .map(|i| {
                lines
                    .iter()
                    .map(|l| l.get(i).unwrap())
                    .filter(|c| **c == '#')
                    .count()
                    - 1
            })
            .collect_vec()
    }

    fn parse(fname: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];

        let contents = read_to_string(fname).unwrap();
        let items = contents.split("\n\n").map(|s| s.trim()).collect_vec();
        for item in items {
            let lines = item.lines().map(|l| l.chars().collect_vec()).collect_vec();
            let counts = Self::parse_item(lines);
            let col = if item.starts_with('#') {
                &mut locks
            } else {
                &mut keys
            };
            col.push(counts);
        }

        Schematics { keys, locks }
    }

    fn fits(key: &Key, lock: &Lock) -> bool {
        key.iter().zip(lock).all(|(k, l)| k + l <= 5)
    }

    fn part1(&self) -> usize {
        let mut fits = 0;
        for k in &self.keys {
            for l in &self.locks {
                if Self::fits(k, l) {
                    fits += 1;
                }
            }
        }
        fits
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let schematics = Schematics::parse(&fname);
    println!("Part 1: {}", schematics.part1());
}
