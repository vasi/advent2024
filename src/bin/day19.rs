use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

struct Onsen {
    patterns: Vec<String>,
    designs: Vec<String>,
}

type Cache = HashMap<String, bool>;

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

    fn possible(patterns: &Vec<String>, cache: &mut Cache, design: &str) -> bool {
        if let Some(v) = cache.get(design) {
            return *v;
        }

        let mut found = false;
        for pat in patterns {
            if let Some(suffix) = design.strip_prefix(pat) {
                if Self::possible(patterns, cache, suffix) {
                    found = true;
                    break;
                }
            }
        }
        cache.insert(design.to_owned(), found);
        found
    }

    fn part1(&self) -> usize {
        let mut cache = Cache::new();
        cache.insert("".to_owned(), true);
        self.designs
            .iter()
            .filter(|d| Self::possible(&self.patterns, &mut cache, d))
            .count()
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let onsen = Onsen::parse(&fname);
    println!("{}", onsen.part1());
}
