use std::collections::HashMap;
use std::env::args;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dir {
    dx: i64,
    dy: i64,
}

impl Dir {
    const LEFT: Self = Self { dx: -1, dy: 0 };
    const RIGHT: Self = Self { dx: 1, dy: 0 };
    const UP: Self = Self { dx: 0, dy: -1 };
    const DOWN: Self = Self { dx: 0, dy: 1 };

    fn parse(c: char) -> Self {
        match c {
            '<' => Self::LEFT,
            '>' => Self::RIGHT,
            '^' => Self::UP,
            'v' => Self::DOWN,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn go(&self, dir: &Dir) -> Self {
        Self::new(self.x + dir.dx, self.y + dir.dy)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Bot,
    Box,
    Wall,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Bot,
            'O' => Self::Box,
            '#' => Self::Wall,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    width: i64,
    height: i64,
    bot: Pos,
    obstacles: HashMap<Pos, Tile>,
    moves: Vec<Dir>,
}

impl Warehouse {
    fn parse(fname: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut obstacles = HashMap::new();
        let mut bot = Pos::new(0, 0);
        let mut moves = Vec::new();
        let mut done_map = false;

        let contents = fs::read_to_string(fname).unwrap();
        for (y, line) in contents.lines().enumerate() {
            if done_map {
                moves.extend(line.chars().map(|c| Dir::parse(c)));
            } else if line.is_empty() {
                done_map = true;
            } else {
                width = line.len() as i64;
                height = (y + 1) as i64;
                for (x, c) in line.chars().enumerate() {
                    match Tile::parse(c) {
                        Tile::Bot => bot = Pos::new(x as i64, y as i64),
                        Tile::Box => {
                            obstacles.insert(Pos::new(x as i64, y as i64), Tile::Box);
                        }
                        Tile::Wall => {
                            obstacles.insert(Pos::new(x as i64, y as i64), Tile::Wall);
                        }
                        Tile::Empty => (),
                    }
                }
            }
        }
        Self {
            width,
            height,
            bot,
            obstacles,
            moves,
        }
    }

    fn move_one(&mut self, dir: &Dir) {
        let bot_pos = self.bot;
        let bot_dest = bot_pos.go(&dir);
        let mut check = bot_dest;
        loop {
            match self.obstacles.get(&check) {
                Some(Tile::Wall) => return,
                Some(Tile::Box) => check = check.go(&dir),
                None => {
                    if check != bot_dest {
                        self.obstacles.insert(check, Tile::Box);
                        self.obstacles.remove(&bot_dest);
                    }
                    self.bot = bot_dest;
                    return;
                }
                _ => unreachable!(),
            }
        }
    }

    fn move_all(&mut self) {
        for dir in self.moves.clone().iter() {
            self.move_one(&dir);
        }
    }

    fn score(&self) -> i64 {
        self.obstacles
            .iter()
            .filter(|(_, &t)| t == Tile::Box)
            .map(|(p, _)| p.x + 100 * p.y)
            .sum()
    }

    fn part1(&mut self) -> i64 {
        self.move_all();
        self.score()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x, y);
                if pos == self.bot {
                    write!(f, "{}", '@')?;
                } else {
                    let c = match self.obstacles.get(&pos) {
                        None => '.',
                        Some(Tile::Box) => 'O',
                        Some(Tile::Wall) => '#',
                        _ => unreachable!(),
                    };
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let mut warehouse = Warehouse::parse(&fname);
    println!("{}", warehouse.part1());
}
