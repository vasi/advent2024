use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct LabeledPos {
    pos: Pos,
    label: char,
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

    fn antinodes_pair<T: Iterator<Item = i64> + Clone>(
        &self,
        range: T,
        near: &LabeledPos,
        far: &LabeledPos,
    ) -> HashSet<LabeledPos> {
        let dx = near.pos.x - far.pos.x;
        let dy = near.pos.y - far.pos.y;
        let mut ret = HashSet::new();

        for m in range {
            let pos = Pos::new(near.pos.x + m * dx, near.pos.y + m * dy);
            if !self.in_bounds(&pos) {
                break;
            }
            ret.insert(LabeledPos {
                pos,
                label: near.label,
            });
        }
        ret
    }

    fn antinodes_all<T: Iterator<Item = i64> + Clone>(&self, range: T) -> HashSet<LabeledPos> {
        let mut ret = HashSet::new();
        let by_freq = self.antennas.iter().into_group_map_by(|a| a.label);
        for aas in by_freq.values() {
            for (a1, a2) in aas.iter().tuple_combinations() {
                ret.extend(self.antinodes_pair(range.clone(), a1, a2));
                ret.extend(self.antinodes_pair(range.clone(), a2, a1));
            }
        }
        ret
    }

    fn solve<T: Iterator<Item = i64> + Clone>(&self, range: T) -> usize {
        self.antinodes_all(range)
            .iter()
            .unique_by(|a| a.pos)
            .count()
    }

    fn print_solution<T: Iterator<Item = i64> + Clone>(&self, range: T) {
        let mut points = HashMap::new();
        for a in self.antinodes_all(range) {
            points.insert(a.pos, '#');
        }
        for a in &self.antennas {
            points.insert(a.pos, a.label);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Pos::new(x as i64, y as i64);
                let c = match points.get(&p) {
                    None => '.',
                    Some(l) => *l,
                };
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let city = City::parse(&fname);
    println!("Part 1: {}", city.solve(1..2));
    println!("Part 2: {}", city.solve(0_i64..));
    city.print_solution(1..2);
}
