use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
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
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
        ]
    }

    fn manhattan(&self, other: &Coord2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

type Path = HashMap<Coord2, i64>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Cheat {
    start: Coord2,
    end: Coord2,
    saved: i64,
}

impl Cheat {
    fn new(path: &Path, p1: &Coord2, p2: &Coord2) -> Option<Self> {
        let d1 = *path.get(&p1).unwrap();
        let d2 = *path.get(&p2).unwrap();
        let (&start, &end) = if d1 < d2 { (p1, p2) } else { (p2, p1) };
        let saved = (d1 - d2).abs() - p1.manhattan(p2);
        if saved <= 0 {
            None
        } else {
            Some(Self { start, end, saved })
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    width: i64,
    height: i64,
    start: Coord2,
    end: Coord2,
    walls: HashSet<Coord2>,
}

impl Maze {
    fn parse(fname: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut start = Coord2::new(0, 0);
        let mut end = Coord2::new(0, 0);
        let mut walls = HashSet::new();

        let contents = read_to_string(fname).unwrap();
        for (y, line) in contents.lines().enumerate() {
            height = (y as i64) + 1;
            width = line.len() as i64;
            for (x, c) in line.chars().enumerate() {
                let pos = Coord2::new(x as i64, y as i64);
                match c {
                    '.' => (),
                    'S' => start = pos,
                    'E' => end = pos,
                    '#' => {
                        walls.insert(pos);
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            width,
            height,
            start,
            end,
            walls,
        }
    }

    fn path(&self) -> Path {
        let mut path = Path::new();
        let mut pos = self.start;
        path.insert(pos, 0);
        'outer: while pos != self.end {
            for adj in pos.adjacent() {
                if !self.walls.contains(&adj) && !path.contains_key(&adj) {
                    path.insert(adj, path.len() as i64);
                    pos = adj;
                    continue 'outer;
                }
            }
            unreachable!()
        }
        path
    }

    // Returned cheats are ordered as (start, end)
    fn find_cheats(&self, path: &Path) -> HashSet<Cheat> {
        let mut cheats = HashSet::new();
        for wall in &self.walls {
            for p1 in wall.adjacent() {
                if !path.contains_key(&p1) {
                    continue;
                }
                for p2 in wall.adjacent() {
                    if p1 == p2 || !path.contains_key(&p2) {
                        continue;
                    }
                    cheats.extend(Cheat::new(path, &p1, &p2));
                }
            }
        }

        cheats
    }

    fn part1(&self) -> usize {
        let path = self.path();
        let cheats = self.find_cheats(&path);
        cheats.iter().filter(|ch| ch.saved >= 100).count()
    }

    fn part2(&self, min_dist: i64) -> usize {
        let mut cheats = HashSet::new();
        let path = self.path();

        for p1 in path.keys() {
            for p2 in path.keys() {
                if p1.manhattan(p2) <= 20 {
                    let cheat = Cheat::new(&path, p1, p2);
                    if let Some(ch) = cheat {
                        if ch.saved >= min_dist {
                            cheats.insert(ch);
                        }
                    }
                }
            }
        }
        cheats.len()
    }
}

#[allow(dead_code)]
fn order_path(path: &Path) -> Vec<Coord2> {
    path.iter()
        .sorted_by_key(|(_, &v)| v)
        .map(|(k, _)| k)
        .copied()
        .collect()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let maze = Maze::parse(&fname);
    println!("Part 1: {}", maze.part1());
    println!("Part 2: {}", maze.part2(100));
}
