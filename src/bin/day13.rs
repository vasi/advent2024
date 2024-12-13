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
    fn is_solution(&self, a: i64, b: i64) -> bool {
        self.a.x * a + self.b.x * b == self.prize.x && self.a.y * a + self.b.y * b == self.prize.y
    }

    fn solutions(&self) -> Vec<Solution> {
        let mut solutions = Vec::new();
        for a in 0..100 {
            for b in 0..100 {
                if self.is_solution(a, b) {
                    solutions.push(Solution { a, b });
                }
            }
        }
        solutions
    }

    fn solve1(&self) -> i64 {
        self.solutions()
            .iter()
            .map(|s| s.score())
            .min()
            .unwrap_or(0)
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
    machines.iter().map(|m| m.solve1()).sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let machines = parse(&fname);
    println!("Part1: {}", part1(&machines));
}
