use image::{GrayImage, Luma};
use itertools::Itertools;
use regex::{Captures, Regex};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::args;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coord2 {
    x: i64,
    y: i64,
}

impl Coord2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn from_caps(caps: &Captures, ix: usize) -> Self {
        Self::new(caps[ix].parse().unwrap(), caps[ix + 1].parse().unwrap())
    }

    fn add_n(&mut self, v: &Self, grid: &Self, n: i64) {
        self.x = (self.x + v.x * n).rem_euclid(grid.x);
        self.y = (self.y + v.y * n).rem_euclid(grid.y);
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Robot {
    position: Coord2,
    velocity: Coord2,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Room {
    size: Coord2,
    robots: Vec<Robot>,
}

impl Room {
    fn parse(fname: &str) -> Self {
        let size_re = Regex::new(r"size=(\d+),(\d+)").unwrap();
        let bot_re = Regex::new(r"p=(\d+),(\d+) v=([-\d]+),([-\d]+)").unwrap();
        let mut size = Coord2::new(0, 0);
        let mut robots = Vec::new();

        let content = read_to_string(fname).unwrap();
        for line in content.lines() {
            if let Some(caps) = size_re.captures(line) {
                size = Coord2::from_caps(&caps, 1);
            } else if let Some(caps) = bot_re.captures(line) {
                let bot = Robot {
                    position: Coord2::from_caps(&caps, 1),
                    velocity: Coord2::from_caps(&caps, 3),
                };
                robots.push(bot);
            } else {
                panic!("Unrecognized line");
            }
        }

        Self { size, robots }
    }

    fn count_by_pos(&self) -> HashMap<Coord2, usize> {
        self.robots.iter().counts_by(|b| b.position)
    }

    fn move_n(&mut self, n: i64) {
        for bot in &mut self.robots {
            bot.position.add_n(&bot.velocity, &self.size, n);
        }
    }

    fn count_by_quadrant(&self) -> Vec<usize> {
        let mut counts = vec![0; 4];
        let mx = self.size.x / 2;
        let my = self.size.y / 2;
        for bot in &self.robots {
            let quadrant = match (bot.position.x.cmp(&mx), bot.position.y.cmp(&my)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Greater, Ordering::Less) => Some(1),
                (Ordering::Less, Ordering::Greater) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            };
            if let Some(q) = quadrant {
                *counts.get_mut(q).unwrap() += 1;
            }
        }
        counts
    }

    fn part1(&mut self) -> usize {
        self.move_n(100);
        self.count_by_quadrant().iter().product()
    }

    fn save_png(&self, fname: &str) {
        let mut img = GrayImage::new(self.size.x as u32, self.size.y as u32);
        for bot in &self.robots {
            img.put_pixel(bot.position.x as u32, bot.position.y as u32, Luma([255]));
        }
        img.save(fname).unwrap();
    }

    fn print_pngs(&mut self) {
        for i in 0..99999 {
            let fname = format!("img/{:0>5}.png", &i);
            self.save_png(&fname);
            self.move_n(1);
        }
        // Vert: 13, 114, 215, 316, ...
        // Horiz: 79, 182, 285, ...
    }

    fn part2(&mut self) {
        for i in 0..999999 {
            if i % 101 == 13 && i % 103 == 79 {
                let fname = format!("img/{:0>5}.png", &i);
                self.save_png(&fname);
                return;
            }
            self.move_n(1);
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let by_pos = self.count_by_pos();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Coord2::new(x, y);
                let c = match by_pos.get(&pos) {
                    Some(cnt) => cnt.to_string(),
                    None => ".".to_owned(),
                };
                f.write_str(&c)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let mut room = Room::parse(&fname);
    println!("Part 1: {}", room.clone().part1());
    room.part2();
}
