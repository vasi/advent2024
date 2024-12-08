use itertools::Itertools;
use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    // The antinode closer to self
    fn antinode(&self, other: &Self) -> Self {
        Self::new(self.x - (other.x - self.x), self.y - (other.y - self.y))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct LabeledPos {
    pos: Pos,
    label: char,
}

impl LabeledPos {
    fn antinode(&self, other: &Self) -> Self {
        Self {
            pos: self.pos.antinode(&other.pos),
            label: self.label,
        }
    }
}

#[derive(Debug, Clone)]
struct City {
    width: usize,
    height: usize,
    antennas: Vec<LabeledPos>,
}

impl City {
    fn parse(fname: &str) -> Self {
        let contents = read_to_string(fname).unwrap();
        let mut width = 0;
        let mut height = 0;
        let mut antennas = Vec::new();
        for (y, line) in contents.lines().enumerate() {
            width = line.len();
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.push(LabeledPos {
                        pos: Pos::new(x as i64, y as i64),
                        label: c,
                    })
                }
            }
        }

        Self {
            width,
            height,
            antennas,
        }
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < (self.width as i64) && pos.y >= 0 && pos.y < (self.height as i64)
    }

    fn antinodes(&self) -> HashSet<LabeledPos> {
        let mut ret = HashSet::new();
        let mut add_antinode = |an: LabeledPos| {
            if self.in_bounds(&an.pos) {
                ret.insert(an);
            }
        };
        let by_freq = self.antennas.iter().into_group_map_by(|a| a.label);
        for aas in by_freq.values() {
            for (a1, a2) in aas.iter().tuple_combinations() {
                add_antinode(a1.antinode(*a2));
                add_antinode(a2.antinode(*a1));
            }
        }
        ret
    }

    fn part1(&self) -> usize {
        self.antinodes().iter().unique_by(|a| a.pos).count()
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let city = City::parse(&fname);
    println!("Part 1: {}", city.part1());
}
