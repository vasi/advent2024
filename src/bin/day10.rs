use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x + 1, self.y),
        ]
    }
}

type Topo = HashMap<Pos, i64>;

fn parse(fname: &str) -> Topo {
    let contents = read_to_string(fname).unwrap();
    let mut topo = Topo::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let h = c.to_digit(10).unwrap();
            topo.insert(Pos::new(x as i64, y as i64), h as i64);
        }
    }
    topo
}

fn trailheads(topo: &Topo) -> Vec<Pos> {
    topo.iter()
        .filter(|(_, h)| **h == 0)
        .map(|(p, _)| *p)
        .collect()
}

fn reachable_tops(topo: &Topo, trailhead: &Pos) -> HashSet<Pos> {
    let mut tops = HashSet::new();
    let mut done = HashSet::new();
    let mut todo: Vec<Pos> = Vec::new();
    todo.push(*trailhead);
    while let Some(p) = todo.pop() {
        if done.contains(&p) {
            continue;
        }
        if let Some(h) = topo.get(&p) {
            if *h == 9 {
                tops.insert(p);
            }
            for adj in p.adjacent() {
                if topo.get(&adj) == Some(&(h + 1)) {
                    todo.push(adj);
                }
            }
        }
        done.insert(p);
    }
    tops
}

fn part1(topo: &Topo) -> i64 {
    trailheads(topo)
        .iter()
        .map(|th| reachable_tops(topo, th).len() as i64)
        .sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let topo = parse(&fname);
    println!("Part 1: {}", part1(&topo));
}
