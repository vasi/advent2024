use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn soluble_inner(&self, idx: usize, val: i64) -> bool {
        match self.operands.get(idx) {
            None => self.result == val,
            Some(n) => self.soluble_inner(idx + 1, val + n) || self.soluble_inner(idx + 1, val * n),
        }
    }

    fn soluble(&self) -> bool {
        self.soluble_inner(1, *self.operands.get(0).unwrap())
    }
}

fn parse(fname: &str) -> Vec<Equation> {
    read_to_string(fname)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse::<i64>().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            Equation { result, operands }
        })
        .collect()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let equations = parse(&fname);

    let mut tot = 0;
    for eq in equations {
        if eq.soluble() {
            tot += eq.result;
        }
    }
    println!("Part 1: {}", tot);
}
