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

    fn path(&self) -> HashMap<Coord2, i64> {
        let mut path = HashMap::new();
        let mut pos = self.start;
        path.insert(pos, 0);
        'outer: while pos != self.end {
            for adj in pos.adjacent() {
                if !self.walls.contains(&adj) && !path.contains_key(&adj) {
                    path.insert(adj, (path.len() as i64) + 1);
                    pos = adj;
                    continue 'outer;
                }
            }
            unreachable!()
        }
        path
    }
}

fn order_path(path: &HashMap<Coord2, i64>) -> Vec<Coord2> {
    path.iter()
        .sorted_by_key(|(_, &v)| v)
        .map(|(k, _)| k)
        .copied()
        .collect()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let maze = Maze::parse(&fname);
    let path = maze.path();
    let ordered_path = order_path(&path);
    assert_eq!(
        maze.width * maze.height,
        (maze.walls.len() + ordered_path.len()) as i64,
    );

    println!("{:?}", &ordered_path);
}
