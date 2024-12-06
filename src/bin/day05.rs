use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

type Edges = Vec<(usize, usize)>;
type Update = Vec<usize>;
type EdgesBy = HashMap<usize, HashSet<usize>>;

fn parse(fname: &str) -> (Edges, Vec<Update>) {
    let contents = read_to_string(fname).unwrap();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    for line in contents.lines() {
        if line.contains('|') {
            let mut parts = line.split('|');
            let bef = parts.next().unwrap().parse().unwrap();
            let aft = parts.next().unwrap().parse().unwrap();
            edges.push((bef, aft));
        } else if line.contains(',') {
            let pages: Vec<usize> = line
                .split(',')
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            updates.push(pages);
        }
    }
    (edges, updates)
}

fn update_ok(update: &Update, edges: &Edges) -> bool {
    // Build edges for just the things we care about
    let pageset: HashSet<&usize> = update.iter().collect();
    let mut before = EdgesBy::new();
    let mut after = EdgesBy::new();
    for (b, a) in edges {
        if pageset.contains(b) && pageset.contains(a) {
            before.entry(*a).or_insert(HashSet::new()).insert(*b);
            after.entry(*b).or_insert(HashSet::new()).insert(*a);
        }
    }

    for page in update {
        if before.get(page).is_some_and(|b| !b.is_empty()) {
            return false; // something must go before us
        }
        // remove edges where this page is before, they're fine now
        if let Some(bs) = after.get(page) {
            for b in bs {
                if let Some(pages) = before.get_mut(&b) {
                    pages.remove(page);
                }
            }
        }
    }

    true
}

fn part1(edges: &Edges, updates: &Vec<Update>) -> usize {
    let mut tot = 0;
    for update in updates {
        if update_ok(update, &edges) {
            let mid = update.get(update.len() / 2).unwrap();
            tot += mid;
        }
    }
    tot
}

fn main() {
    let fname = args().nth(1).unwrap();
    let (edges, updates) = parse(&fname);
    println!("Part 1: {}", part1(&edges, &updates))
}
