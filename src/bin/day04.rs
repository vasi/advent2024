use anyhow::{bail, Result};
use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Dir {
    dx: i64,
    dy: i64,
}

impl Dir {
    const ALL: [Self; 8] = [
        Self { dx: 1, dy: 0 },
        Self { dx: 0, dy: 1 },
        Self { dx: -1, dy: 0 },
        Self { dx: 0, dy: -1 },
        Self { dx: 1, dy: 1 },
        Self { dx: -1, dy: 1 },
        Self { dx: 1, dy: -1 },
        Self { dx: -1, dy: -1 },
    ];

    const DIAGONAL: [Self; 4] = [
        Self { dx: 1, dy: 1 },
        Self { dx: -1, dy: 1 },
        Self { dx: 1, dy: -1 },
        Self { dx: -1, dy: -1 },
    ];

    fn rotate90(&self) -> Self {
        Self {
            dx: -self.dy,
            dy: self.dx,
        }
    }

    fn invert(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn next(&self, dir: &Dir) -> Self {
        Self {
            x: self.x + dir.dx,
            y: self.y + dir.dy,
        }
    }
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(fname: &str) -> Result<Self> {
        let s = read_to_string(fname)?;
        let mut rows = Vec::new();
        let mut width = None;
        for line in s.lines() {
            let row: Vec<char> = line.chars().collect();
            let row_width = row.len();
            match width {
                Some(w) if w != row_width => bail!("All rows must have the same width"),
                _ => width = Some(row_width),
            }
            rows.push(row);
        }
        Ok(Grid { rows })
    }

    fn width(&self) -> i64 {
        self.rows[0].len() as i64
    }

    fn height(&self) -> i64 {
        self.rows.len() as i64
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width() && pos.y < self.height()
    }

    fn get(&self, pos: &Pos) -> Option<char> {
        if self.in_bounds(pos) {
            Some(self.rows[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn positions(&self) -> PosIter {
        PosIter {
            grid: self,
            pos: Pos { x: 0, y: 0 },
        }
    }

    fn check_word_at(&self, word: &str, pos: &Pos, dir: &Dir) -> bool {
        let mut pos = pos.clone();
        for c in word.chars() {
            if self.get(&pos) != Some(c) {
                return false;
            }
            pos = pos.next(dir);
        }
        true
    }

    fn count_word(&self, word: &str) -> i64 {
        let mut tot = 0;
        for pos in self.positions() {
            for dir in &Dir::ALL {
                if self.check_word_at(word, &pos, &dir) {
                    tot += 1;
                }
            }
        }
        tot
    }

    fn part1(&self) -> i64 {
        self.count_word("XMAS")
    }

    fn check_cross_mas_at(&self, pos: &Pos, dir: &Dir) -> bool {
        let d90 = dir.rotate90();
        self.get(pos) == Some('A')
            && self.get(&pos.next(dir)) == Some('M')
            && self.get(&pos.next(&d90)) == Some('M')
            && self.get(&pos.next(&dir.invert())) == Some('S')
            && self.get(&pos.next(&d90.invert())) == Some('S')
    }

    fn part2(&self) -> i64 {
        let mut tot = 0;
        for pos in self.positions() {
            for dir in &Dir::DIAGONAL {
                if self.check_cross_mas_at(&pos, &dir) {
                    tot += 1;
                }
            }
        }
        tot
    }
}

struct PosIter<'a> {
    grid: &'a Grid,
    pos: Pos,
}

impl<'a> Iterator for PosIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.x += 1;
        if self.pos.x >= self.grid.width() {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        if self.pos.y >= self.grid.height() {
            None
        } else {
            Some(self.pos)
        }
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let grid = Grid::new(&fname).unwrap();
    println!("Part 1: {}", grid.part1());
    println!("Part 2: {}", grid.part2());
}
