use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;
use std::hash::Hash;

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

impl Guard {
  fn next_pos(&self) -> Pos {
    self.pos.next(&self.dir)
  }
  fn turn_right(&mut self) {
    self.dir = self.dir.turn_right();
  }
}

type Obstacles = HashSet<Pos>;

#[derive(Debug, Clone)]
struct Arena {
  width: usize,
  height: usize,
  guard: Guard,
  guard_orig: Pos,
  obstacles: Obstacles,
  visited: HashSet<Guard>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SimulationResult {
  Repeat,
  Exited,
}

impl Arena {
  fn new(width: usize, height: usize, guard_pos: Pos, obstacles: Obstacles) -> Self {
    let mut ret = Self {
      width,
      height,
      guard: Guard { pos: guard_pos, dir: Dir::UP },
      guard_orig: guard_pos,
      obstacles,
      visited: HashSet::new(),
    };
    ret.visited.insert(ret.guard);
    ret
  }

  fn contains(&self, pos: &Pos) -> bool {
    pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
  }

  fn simulate(&mut self) -> SimulationResult {
    loop {
      while self.obstacles.contains(&self.guard.next_pos()) {
        self.guard.turn_right();
      }
      self.guard.pos = self.guard.next_pos();

      if self.visited.contains(&self.guard) {
        return SimulationResult::Repeat;
      } else if !self.contains(&self.guard.pos) {
        return SimulationResult::Exited;
      }
      self.visited.insert(self.guard);
    }
  }

  fn with_obstacle(&self, pos: &Pos) -> Self {
    let mut arena = Self {
      width: self.width,
      height: self.height,
      guard: Guard { pos: self.guard_orig, dir: Dir::UP },
      guard_orig: self.guard_orig,
      obstacles: self.obstacles.clone(),
      visited: HashSet::new(),
    };
    arena.obstacles.insert(*pos);
    arena
  }

  fn visited_positions(&self) -> HashSet<Pos> {
    self.visited.iter().map(|g| g.pos).collect()
  }
}

fn parse(fname: &str) -> Arena {
  let contents = read_to_string(fname).unwrap();

  let mut width = 0;
  let mut height = 0;
  let mut guard_pos = Pos::empty();
  let mut obstacles = Obstacles::new();

  for (y, line) in contents.lines().enumerate() {
    width = line.len();
    for (x, c) in line.chars().enumerate() {
      if c == '#' {
        obstacles.insert(Pos::new(x, y));
      } else if c == '^' {
        guard_pos = Pos::new(x, y);
      }
    }
    height = y + 1;
    }
  Arena::new(width, height, guard_pos, obstacles)
}

fn main() {
  let fname = args().nth(1).unwrap();

  let mut arena = parse(&fname);
  let orig_pos = arena.guard.pos.clone();
  arena.simulate();
  let visited = arena.visited_positions();
  println!("Part 1: {}", visited.len());

  let part2 = visited.iter().filter(|p| {
    if **p == orig_pos {
      false
    } else {
      let mut arena2 = arena.with_obstacle(p);
      arena2.simulate() == SimulationResult::Repeat
    }
  }).count();
  println!("Part 2: {}", part2);
}
