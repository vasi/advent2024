use pathfinding::prelude::astar_bag_collect;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env::args;
use std::fmt::{Display, Formatter};
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

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match (self.dx, self.dy) {
            (1, 0) => 'E',
            (-1, 0) => 'W',
            (0, 1) => 'S',
            (0, -1) => 'N',
            _ => unreachable!(),
        };
        write!(f, "{}", c)
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
        (self.x - other.x).abs() + (self.x - other.y).abs()
    }
}

impl Display for Coord2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Pos {
    coord: Coord2,
    dir: Dir,
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.coord, self.dir)
    }
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

type Visited = HashSet<Coord2>;

#[derive(PartialEq, Eq, Clone)]
struct MinHeapEntry<T> {
    val: T,
    prio: Cost,
}

impl<T> MinHeapEntry<T> {
    fn new(val: T, prio: Cost) -> Self {
        Self { val, prio }
    }
}

impl<T: Ord + Eq> Ord for MinHeapEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prio
            .cmp(&other.prio)
            .reverse()
            .then(self.val.cmp(&other.val))
    }
}

impl<T: Ord + Eq> PartialOrd for MinHeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
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

    #[allow(dead_code)]
    fn show_heap(orig: &BinaryHeap<MinHeapEntry<Pos>>) {
        let mut h = orig.clone();
        while let Some(e) = h.pop() {
            println!("{} {}", e.prio, e.val);
        }
    }

    // Prove that I can implement astar_bag
    fn my_solve(&self) -> (Cost, Visited) {
        let start = Pos::new(self.start, Dir::EAST);

        let mut todo = BinaryHeap::new();
        let mut best = HashMap::new();
        let mut came_from: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut results = HashSet::new();
        let mut best_result = None;
        todo.push(MinHeapEntry::new(start, 0));
        best.insert(start, 0);
        came_from.insert(start, vec![]);

        while let Some(cur) = todo.pop() {
            let cur_cost = *best.get(&cur.val).unwrap();
            if cur.val.coord == self.end {
                results.insert(cur.val);
                best_result.get_or_insert(cur_cost);
                continue;
            }
            if best_result.is_some_and(|b| b < cur_cost) {
                break;
            }
            for (succ, move_cost) in self.successors(&cur.val) {
                let new_cost = cur_cost + move_cost;
                let prev_best = best.get(&succ);
                if prev_best == Some(&new_cost) {
                    came_from.get_mut(&succ).unwrap().push(cur.val);
                } else if prev_best.map(|b| *b > new_cost).unwrap_or(true) {
                    best.insert(succ, new_cost);
                    came_from.insert(succ, vec![cur.val]);
                    let prio = new_cost + self.end.manhattan(&succ.coord);
                    todo.push(MinHeapEntry::new(succ, prio));
                }
            }
        }

        let mut visited_pos = HashSet::new();
        let mut visited_todo = results.iter().collect::<Vec<_>>();
        while let Some(t) = visited_todo.pop() {
            if visited_pos.insert(t) {
                visited_todo.extend(came_from.get(t).unwrap());
            }
        }
        let visited = visited_pos.iter().map(|p| p.coord).collect::<HashSet<_>>();
        (best_result.unwrap(), visited)
    }

    #[allow(dead_code)]
    fn solve(&self) -> (Cost, Visited) {
        let start = Pos::new(self.start, Dir::EAST);
        let (paths, cost) = astar_bag_collect(
            &start,
            |p| self.successors(p),
            |p| self.end.manhattan(&p.coord),
            |p| p.coord == self.end,
        )
        .unwrap();
        let visited: HashSet<Coord2> = paths.iter().flat_map(|p| p).map(|p| p.coord).collect();
        (cost, visited)
    }

    #[allow(dead_code)]
    fn print_visited(&self, vis: &Visited) {
        let width = self.walls.iter().map(|w| w.x).max().unwrap() + 1;
        let height = self.walls.iter().map(|w| w.y).max().unwrap() + 1;
        for y in 0..height {
            for x in 0..width {
                let c = Coord2::new(x, y);
                let p = if vis.contains(&c) {
                    'O'
                } else if self.walls.contains(&c) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", p);
            }
            println!();
        }
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let maze = Maze::parse(&fname);
    let (cost, visited) = maze.my_solve();
    println!("Part 1: {}", cost);
    println!("Part 2: {}", visited.len());
}
