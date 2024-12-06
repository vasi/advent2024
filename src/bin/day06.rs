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

type Obstacles = HashSet<Pos>;

fn parse(fname: &str) -> Arena {
  let contents = read_to_string(fname).unwrap();

  let mut width = 0;
  let mut height = 0;
  let mut guard = Guard { pos: Pos { x: 0, y: 0 }, dir: Dir::UP };
  let mut obstacles = Obstacles::new();

  for (y, line) in contents.lines().enumerate() {
    width = line.len();
    for (x, c) in line.chars().enumerate() {
      if c == '#' {
        obstacles.insert(Pos { x: x as i32, y: y as i32 });
      } else if c == '^' {
        guard.pos = Pos { x: x as i32, y: y as i32 };
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

fn main() {
  let fname = args().nth(1).unwrap();
  let arena = parse(&fname);
  println!("Part 1: {}", part1(arena));
}
