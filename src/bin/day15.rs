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

    fn is_vertical(&self) -> bool {
        self.dx == 0
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = *self;
        write!(
            f,
            "{}",
            match d {
                Dir::LEFT => "left",
                Dir::RIGHT => "right",
                Dir::UP => "up",
                Dir::DOWN => "down",
                _ => unreachable!(),
            }
        )
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
    BoxLeft,
    BoxRight,
    Wall,
}

impl Tile {
    fn parse(c: char, wide: i64) -> Vec<Self> {
        match (c, wide) {
            ('.', 1) => vec![Self::Empty],
            ('@', 1) => vec![Self::Bot],
            ('O', 1) => vec![Self::Box],
            ('#', 1) => vec![Self::Wall],
            ('.', 2) => vec![Self::Empty, Self::Empty],
            ('@', 2) => vec![Self::Bot, Self::Empty],
            ('O', 2) => vec![Self::BoxLeft, Self::BoxRight],
            ('#', 2) => vec![Self::Wall, Self::Wall],
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
    fn parse(fname: &str, wide: i64) -> Self {
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
                height = (y + 1) as i64;
                let mut x = 0;
                for c in line.chars() {
                    let tiles = Tile::parse(c, wide);
                    for t in tiles {
                        match t {
                            Tile::Bot => bot = Pos::new(x as i64, y as i64),
                            Tile::Empty => (),
                            _ => {
                                obstacles.insert(Pos::new(x as i64, y as i64), t);
                            }
                        }
                        x += 1;
                        width = x;
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

    fn need_move(&self, pos: &Pos, dir: &Dir, t: Tile) -> Vec<Pos> {
        match (t, dir.is_vertical()) {
            (Tile::Box, _) | (_, false) => vec![*pos],
            (Tile::BoxLeft, true) => vec![*pos, pos.go(&Dir::RIGHT)],
            (Tile::BoxRight, true) => vec![*pos, pos.go(&Dir::LEFT)],
            _ => unreachable!(),
        }
    }

    fn move_one(&mut self, dir: &Dir) {
        let mut to_move = Vec::new();
        let mut check = Vec::new();
        let bot_dest = self.bot.go(dir);
        check.push(bot_dest);

        loop {
            let mut new_check = Vec::new();
            for pos in check.iter() {
                if let Some(t) = self.obstacles.get(pos) {
                    if t == &Tile::Wall {
                        return;
                    }
                    for np in self.need_move(pos, dir, *t) {
                        to_move.push((np, *self.obstacles.get(&np).unwrap()));
                        new_check.push(np.go(dir));
                    }
                }
            }
            if new_check.is_empty() {
                break;
            }
            check = new_check;
        }

        for (pos, _) in to_move.iter() {
            self.obstacles.remove(&pos);
        }
        for (pos, t) in to_move.iter() {
            self.obstacles.insert(pos.go(dir), *t);
        }
        self.bot = bot_dest;
    }

    fn move_all(&mut self) {
        for dir in self.moves.clone().iter() {
            // println!("{}", dir);
            self.move_one(&dir);
            // print!("{}", self);
        }
    }

    fn score(&self) -> i64 {
        self.obstacles
            .iter()
            .filter(|(_, &t)| t == Tile::Box || t == Tile::BoxLeft)
            .map(|(p, _)| p.x + 100 * p.y)
            .sum()
    }

    fn result(&mut self) -> i64 {
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
                        Some(Tile::BoxLeft) => '[',
                        Some(Tile::BoxRight) => ']',
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

    let mut warehouse = Warehouse::parse(&fname, 1);
    println!("{}", warehouse.result());

    let mut warehouse = Warehouse::parse(&fname, 2);
    println!("{}", warehouse.result());
}
