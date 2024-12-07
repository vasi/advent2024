use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn concat(a: i64, b: i64) -> i64 {
    let bdigits = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_i64.pow(bdigits) + b
}

impl Equation {
    fn soluble_inner(&self, allow_concat: bool, idx: usize, val: i64) -> bool {
        match self.operands.get(idx) {
            None => self.result == val,
            Some(n) => {
                let mut nvs = vec![val + n, val * n];
                if allow_concat {
                    nvs.push(concat(val, *n))
                }
                nvs.iter()
                    .any(|nv| self.soluble_inner(allow_concat, idx + 1, *nv))
            }
        }
    }

    fn soluble(&self, allow_concat: bool) -> bool {
        self.soluble_inner(allow_concat, 1, *self.operands.get(0).unwrap())
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

fn solve(equations: &Vec<Equation>, allow_concat: bool) -> i64 {
    let mut tot = 0;
    for eq in equations {
        if eq.soluble(allow_concat) {
            tot += eq.result;
        }
    }
    tot
}

fn main() {
    let fname = args().nth(1).unwrap();
    let equations = parse(&fname);

    println!("Part 1: {}", solve(&equations, false));
    println!("Part 2: {}", solve(&equations, true));
}
