use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Dir {
  dx: i32,
  dy: i32,
}

impl Dir {
  const UP: Self = Self { dx: 0, dy: -1 };

  fn turn_right(&self) -> Self {
    Self { dx: -self.dy, dy: self.dx }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
  x: i32,
  y: i32,
}

impl Pos {
  fn new(x: usize, y: usize) -> Self {
    Self { x: x as i32, y: y as i32 }
  }

  fn empty() -> Self {
    Self { x: 0, y: 0 }
  }

  fn next(&self, dir: &Dir) -> Self {
    Self {
      x: self.x + dir.dx,
      y: self.y + dir.dy,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
  pos: Pos,
  dir: Dir,
}

type Obstacles = HashSet<Pos>;

#[derive(Debug, Clone)]
struct Arena {
  width: usize,
  height: usize,
  guard: Guard,
  obstacles: Obstacles,
}

impl Arena {
  fn contains(&self, pos: &Pos) -> bool {
    pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
  }
}

struct ObstacleCount<'a> {
  arena: &'a Arena,
  missing: HashSet<Pos>,
  guard: bool,
}

impl <'a> ObstacleCount<'a> {
  fn new(arena: &'a Arena) -> Self {
    Self { arena, missing: HashSet::new(), guard: false }
  }

  fn check(&self, x: usize, y: usize) -> Self {
    let mut guard = self.guard;
    let mut missing = self.missing.clone();
    let pos = Pos::new(x, y);
    if self.arena.guard.pos == pos {
      guard = true;
    }
    if !self.arena.obstacles.contains(&pos) {
      missing.insert(pos);
    }
    Self { arena: self.arena, missing, guard}
  }

  fn complete(&self) -> bool {
    self.missing.len() == 1 && !self.guard
  }
}


fn parse(fname: &str) -> Arena {
  let contents = read_to_string(fname).unwrap();

  let mut width = 0;
  let mut height = 0;
  let mut guard = Guard { pos: Pos::empty(), dir: Dir::UP };
  let mut obstacles = Obstacles::new();

  for (y, line) in contents.lines().enumerate() {
    width = line.len();
    for (x, c) in line.chars().enumerate() {
      if c == '#' {
        obstacles.insert(Pos::new(x, y));
      } else if c == '^' {
        guard.pos = Pos::new(x, y);
      }
    }
    height = y + 1;
  }
  Arena { width, height, guard, obstacles }
}

fn part1(mut arena: Arena) -> usize {
  let mut visited = HashSet::new();
  visited.insert(arena.guard.pos);

  loop {
    let next_pos = arena.guard.pos.next(&arena.guard.dir);
    if arena.obstacles.contains(&next_pos) {
      arena.guard.dir = arena.guard.dir.turn_right();
    } else if arena.contains(&next_pos) {
      arena.guard.pos = next_pos;
      visited.insert(next_pos);
    } else {
      break;
    }
  }
  visited.len()
}

fn part2(arena: &Arena) -> usize {
  let mut found = HashSet::new();
  for y1 in 0..(arena.height-1) {
    for x1 in 1..arena.width {
      let oc = ObstacleCount::new(&arena).check(x1, y1);
      for x2 in (x1+1)..arena.width {
        let oc2 = oc.check(x2, y1 + 1);
        if oc2.missing.len() > 1 {
          break;
        }
        for y2 in (y1+2)..arena.height {
          let oc3 = oc2.check(x2 - 1, y2).check(x1-1, y2 - 1);
          if oc3.complete() {
            found.insert(oc3.missing.iter().next().unwrap().clone());
          }
        }
      }
    }
  }

  found.len()
}

fn main() {
  let fname = args().nth(1).unwrap();
  let arena = parse(&fname);
  println!("Part 1: {}", part1(arena.clone()));
  println!("Part 2: {}", part2(&arena));
}
