use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Dir {
    dx: i64,
    dy: i64,
}

impl Dir {
    const EAST: Self = Self { dx: 1, dy: 0 };

    fn left(&self) -> Self {
        Dir {
            dx: self.dy,
            dy: -self.dx,
        }
    }
    fn right(&self) -> Self {
        Dir {
            dx: -self.dy,
            dy: self.dx,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coord2 {
    x: i64,
    y: i64,
}

impl Coord2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn go(&self, dir: &Dir) -> Self {
        Self::new(self.x + dir.dx, self.y + dir.dy)
    }

    fn manhattan(&self, other: &Coord2) -> i64 {
        return (self.x - other.x).abs() + (self.x - other.y).abs();
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Pos {
    coord: Coord2,
    dir: Dir,
}

type Cost = i64;

impl Pos {
    fn new(coord: Coord2, dir: Dir) -> Self {
        Self { coord, dir }
    }

    fn forward(&self) -> Self {
        Self::new(self.coord.go(&self.dir), self.dir)
    }
    fn left(&self) -> Self {
        Self::new(self.coord, self.dir.left())
    }
    fn right(&self) -> Self {
        Self::new(self.coord, self.dir.right())
    }

    fn successors(&self) -> Vec<(Pos, Cost)> {
        vec![
            (self.forward(), 1),
            (self.left(), 1000),
            (self.right(), 1000),
        ]
    }
}

struct Maze {
    start: Coord2,
    end: Coord2,
    walls: HashSet<Coord2>,
}

impl Maze {
    fn parse(fname: &str) -> Self {
        let contents = read_to_string(fname).unwrap();

        let mut start = Coord2::new(0, 0);
        let mut end = Coord2::new(0, 0);
        let mut walls = HashSet::new();

        for (y, line) in contents.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord2::new(x as i64, y as i64);
                match c {
                    '#' => {
                        walls.insert(coord);
                    }
                    'S' => start = coord,
                    'E' => end = coord,
                    _ => (),
                }
            }
        }

        Self { start, end, walls }
    }

    fn successors(&self, pos: &Pos) -> Vec<(Pos, Cost)> {
        pos.successors()
            .iter()
            .copied()
            .filter(|(p, _)| !self.walls.contains(&p.coord))
            .collect()
    }

    fn solve(&self) -> Cost {
        let start = Pos::new(self.start, Dir::EAST);
        let result = astar(
            &start,
            |p| self.successors(p),
            |p| self.end.manhattan(&p.coord),
            |p| p.coord == self.end,
        );
        result.unwrap().1
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let maze = Maze::parse(&fname);
    println!("Part 1: {}", maze.solve());
}
