use std::collections::HashSet;
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
}

fn main() {
    let fname = args().nth(1).unwrap();
    let maze = Maze::parse(&fname);
    println!("{:#?}", &maze);
}
