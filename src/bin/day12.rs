use std::collections::{BTreeSet, HashMap};
use std::env::args;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

type Garden = HashMap<Pos, char>;
type Plot = BTreeSet<Pos>;

fn parse(fname: &str) -> Garden {
    let mut garden = Garden::new();
    let content = read_to_string(fname).unwrap();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            garden.insert(Pos::new(x as i64, y as i64), c);
        }
    }
    garden
}

fn plot(garden: &Garden, avail: &BTreeSet<Pos>, start: &Pos) -> Plot {
    let c = garden.get(start);
    let mut plot = Plot::new();
    plot.insert(*start);
    let mut todo = Vec::new();
    todo.push(*start);

    while let Some(p) = todo.pop() {
        for adj in p.adjacent() {
            if avail.contains(&adj) && !plot.contains(&adj) && garden.get(&adj) == c {
                plot.insert(adj);
                todo.push(adj);
            }
        }
    }
    plot
}

fn plots(garden: &Garden) -> Vec<Plot> {
    let mut plots = Vec::new();
    let mut avail: BTreeSet<Pos> = garden.keys().map(|k| *k).collect();
    while !avail.is_empty() {
        let pos = avail.iter().next().unwrap();
        let plot = plot(garden, &avail, pos);
        avail.retain(|p| !plot.contains(p));
        plots.push(plot);
    }
    plots
}

fn perimeter(plot: &Plot) -> i64 {
    let mut tot = 0;
    for p in plot.iter() {
        for adj in p.adjacent() {
            if !plot.contains(&adj) {
                tot += 1;
            }
        }
    }
    tot
}

fn cost(plot: &Plot) -> i64 {
    (plot.len() as i64) * perimeter(plot)
}

fn part1(garden: &Garden) -> i64 {
    let plots = plots(garden);
    plots.iter().map(|p| cost(p)).sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let garden = parse(&fname);
    println!("Part 1: {}", part1(&garden));
}
