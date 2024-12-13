use rational::{self, Rational};
use regex::Regex;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Solution {
    a: i64,
    b: i64,
}

impl Solution {
    fn score(&self) -> i64 {
        3 * self.a + self.b
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    fn solution(&self) -> Option<Solution> {
        let ax = Rational::integer(self.a.x as i128);
        let ay = Rational::integer(self.a.y as i128);
        let bx = Rational::integer(self.b.x as i128);
        let by = Rational::integer(self.b.y as i128);
        let px = Rational::integer(self.prize.x as i128);
        let py = Rational::integer(self.prize.y as i128);

        let nb = (py * ax - px * ay) / (by * ax - bx * ay);
        let na = (px - bx * nb) / ax;
        if nb.is_integer() && na.is_integer() {
            Some(Solution {
                a: na.numerator() as i64,
                b: nb.numerator() as i64,
            })
        } else {
            None
        }
    }

    fn score(&self) -> i64 {
        self.solution().map(|s| s.score()).unwrap_or(0)
    }

    fn part2(&self) -> Self {
        let mut r = self.clone();
        r.prize.x += 10000000000000;
        r.prize.y += 10000000000000;
        r
    }
}

fn parse(fname: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let contents = read_to_string(fname).unwrap();

    re.captures_iter(&contents)
        .map(|cap| Machine {
            a: Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            b: Coord::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            prize: Coord::new(cap[5].parse().unwrap(), cap[6].parse().unwrap()),
        })
        .collect()
}

fn part1(machines: &Vec<Machine>) -> i64 {
    machines.iter().map(|m| m.score()).sum()
}

fn part2(machines: &Vec<Machine>) -> i64 {
    machines.iter().map(|m| m.part2().score()).sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let machines = parse(&fname);
    println!("Part1: {}", part1(&machines));
    println!("Part2: {}", part2(&machines));
}
