use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

type Edges = Vec<(usize, usize)>;
type Update = Vec<usize>;
type EdgesBy = HashMap<usize, HashSet<usize>>;

struct Topo {
    before: EdgesBy,
    after: EdgesBy,
}

impl Topo {
    fn new_edges(update: &Update) -> EdgesBy {
        update.iter().map(|p| (*p, HashSet::new())).collect()
    }

    fn new(edges: &Edges, update: &Update) -> Self {
        let pageset: HashSet<&usize> = update.iter().collect();
        let mut before = Self::new_edges(update);
        let mut after = Self::new_edges(update);
        for (b, a) in edges {
            if pageset.contains(b) && pageset.contains(a) {
                before.get_mut(a).unwrap().insert(*b);
                after.get_mut(b).unwrap().insert(*a);
            }
        }
        Self { before, after }
    }

    fn remove_page(&mut self, page: usize) {
        let befores = self.after.get(&page).unwrap();
        for b in befores {
            if let Some(pages) = self.before.get_mut(&b) {
                pages.remove(&page);
            }
        }
    }

    fn ordered(&mut self) -> Update {
        let mut ret = Update::new();

        while !self.before.is_empty() {
            let (&page, _) = self.before.iter().find(|(_, v)| v.is_empty()).unwrap();
            ret.push(page);
            self.before.remove(&page);
            self.remove_page(page);
        }
        ret
    }
}

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

fn main() {
    let fname = args().nth(1).unwrap();
    let (edges, updates) = parse(&fname);

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        let mut topo = Topo::new(&edges, &update);
        let ordered = topo.ordered();
        let mid = ordered.get(ordered.len() / 2).unwrap();
        if ordered == update {
            part1 += mid;
        } else {
            part2 += mid;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
