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

    fn go(&self, dir: &Dir) -> Self {
        Self::new(self.x + dir.dx, self.y + dir.dy)
    }

    fn adjacent(&self) -> Vec<Pos> {
        Dir::ALL.iter().map(|d| self.go(&d)).collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Dir {
    dx: i64,
    dy: i64,
}

impl Dir {
    const ALL: [Self; 4] = [
        Self { dx: 1, dy: 0 },
        Self { dx: 0, dy: 1 },
        Self { dx: -1, dy: 0 },
        Self { dx: 0, dy: -1 },
    ];
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

fn region<GroupFn: Fn(&Pos, &Pos) -> bool>(
    avail: &BTreeSet<Pos>,
    start: &Pos,
    gf: &GroupFn,
) -> BTreeSet<Pos> {
    let mut result = BTreeSet::new();
    result.insert(*start);
    let mut todo = Vec::new();
    todo.push(*start);

    while let Some(p) = todo.pop() {
        for adj in p.adjacent() {
            if avail.contains(&adj) && !result.contains(&adj) && gf(start, &adj) {
                result.insert(adj);
                todo.push(adj);
            }
        }
    }
    result
}

fn regions<GroupFn: Fn(&Pos, &Pos) -> bool>(
    mut avail: BTreeSet<Pos>,
    gf: &GroupFn,
) -> Vec<BTreeSet<Pos>> {
    let mut regions = Vec::new();
    while !avail.is_empty() {
        let pos = avail.iter().next().unwrap();
        let plot = region(&avail, pos, gf);
        avail.retain(|p| !plot.contains(p));
        regions.push(plot);
    }
    regions
}

fn plots(garden: &Garden) -> Vec<Plot> {
    let avail: BTreeSet<Pos> = garden.keys().map(|k| *k).collect();
    regions(avail, &|a, b| garden.get(a) == garden.get(b))
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

fn sides(plot: &Plot) -> i64 {
    let mut by_dir: HashMap<Dir, BTreeSet<Pos>> =
        Dir::ALL.iter().map(|d| (*d, BTreeSet::new())).collect();
    for start in plot {
        for dir in Dir::ALL {
            let target = start.go(&dir);
            if !plot.contains(&target) {
                by_dir.get_mut(&dir).unwrap().insert(target);
            }
        }
    }
    by_dir
        .values()
        .map(|dp| regions(dp.clone(), &|_, _| true).len() as i64)
        .sum()
}

fn cost2(plot: &Plot) -> i64 {
    (plot.len() as i64) * sides(plot)
}

fn part2(garden: &Garden) -> i64 {
    let plots = plots(garden);
    plots.iter().map(|p| cost2(p)).sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let garden = parse(&fname);
    println!("Part 1: {}", part1(&garden));
    println!("Part 1: {}", part2(&garden));
}
