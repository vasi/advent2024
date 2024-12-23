use std::env::args;
use std::fs::read_to_string;

fn parse(fname: &str) -> Vec<i64> {
    let contents = read_to_string(fname).unwrap();
    contents.lines().map(|i| i.parse().unwrap()).collect()
}

fn mix_prune(orig: i64, new: i64) -> i64 {
    (orig ^ new) % 16777216
}

fn next(mut n: i64) -> i64 {
    n = mix_prune(n, n * 64);
    n = mix_prune(n, n / 32);
    mix_prune(n, n * 2048)
}

fn nth(mut n: i64, times: usize) -> i64 {
    for _ in 0..times {
        n = next(n);
    }
    n
}

fn part1(initials: &Vec<i64>) -> i64 {
    let mut tot = 0;
    for i in initials {
        tot += nth(*i, 2000);
    }
    tot
}

fn main() {
    let fname = args().nth(1).unwrap();
    let initials = parse(&fname);
    println!("Part 1: {}", part1(&initials));
}
