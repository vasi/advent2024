use pathfinding::prelude::astar;
use regex::Regex;
use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord2 {
    x: i64,
    y: i64,
}

impl Coord2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }

    fn manhattan(&self, o: &Coord2) -> i64 {
        (self.x - o.x).abs() + (self.y - o.y).abs()
    }
}

#[derive(Debug)]
struct Memory {
    width: i64,
    height: i64,
    take: usize,
    bytes: Vec<Coord2>,
}

impl Memory {
    fn parse(fname: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut take = 0;
        let mut bytes = vec![];

        let size_re = Regex::new(r"size:(\d+),(\d+)").unwrap();
        let take_re = Regex::new(r"take:(\d+)").unwrap();
        let contents = read_to_string(fname).unwrap();
        for line in contents.lines() {
            if let Some(caps) = size_re.captures(line) {
                width = caps[1].parse().unwrap();
                height = caps[2].parse().unwrap();
            } else if let Some(caps) = take_re.captures(line) {
                take = caps[1].parse().unwrap();
            } else {
                let (x, y) = line.split_once(",").unwrap();
                let coord = Coord2::new(x.parse().unwrap(), y.parse().unwrap());
                bytes.push(coord);
            }
        }

        Self {
            width,
            height,
            take,
            bytes,
        }
    }

    fn in_bounds(&self, c: &Coord2) -> bool {
        c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height
    }

    fn successors(&self, obstacles: &HashSet<&Coord2>, c: &Coord2) -> Vec<(Coord2, i64)> {
        c.adjacent()
            .iter()
            .copied()
            .filter(|e| self.in_bounds(e) && !obstacles.contains(e))
            .map(|e| (e, 1))
            .collect()
    }

    fn part1(&self) -> i64 {
        let start = Coord2::new(0, 0);
        let target = Coord2::new(self.width - 1, self.height - 1);
        let obstacles: HashSet<_> = self.bytes.iter().take(self.take).collect();

        let (_, cost) = astar(
            &start,
            |c| self.successors(&obstacles, c),
            |c| target.manhattan(c),
            |c| *c == target,
        )
        .unwrap();
        cost
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let memory = Memory::parse(&fname);
    println!("Part 1: {}", memory.part1());
}
