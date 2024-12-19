use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

struct Onsen {
    patterns: Vec<String>,
    designs: Vec<String>,
}

type Cache = HashMap<String, usize>;

impl Onsen {
    fn parse(fname: &str) -> Self {
        let content = read_to_string(fname).unwrap();
        let mut lines = content.lines();
        let patterns = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        let mut designs = vec![];
        for line in lines {
            if !line.is_empty() {
                designs.push(line.to_owned());
            }
        }

        Self { patterns, designs }
    }

    fn possible(patterns: &Vec<String>, cache: &mut Cache, design: &str) -> usize {
        if let Some(v) = cache.get(design) {
            return *v;
        }

        let mut total = 0;
        for pat in patterns {
            if let Some(suffix) = design.strip_prefix(pat) {
                let count = Self::possible(patterns, cache, suffix);
                total += count;
            }
        }
        cache.insert(design.to_owned(), total);
        total
    }

    fn counts(&self) -> Vec<usize> {
        let mut cache = Cache::new();
        cache.insert("".to_owned(), 1);
        self.designs
            .iter()
            .map(|d| Self::possible(&self.patterns, &mut cache, d))
            .collect()
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let onsen = Onsen::parse(&fname);
    let counts = onsen.counts();
    println!("Part 1: {}", counts.iter().filter(|c| **c > 0).count());
    println!("Part 2: {}", counts.iter().copied().sum::<usize>());
}
