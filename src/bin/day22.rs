use itertools::Itertools;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::iter::successors;

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

fn secrets(initial: i64) -> impl Iterator<Item = i64> {
    successors(Some(initial), |i| Some(next(*i))).take(2001)
}

fn part1(initials: &Vec<i64>) -> i64 {
    let mut tot = 0;
    for i in initials {
        tot += secrets(*i).last().unwrap()
    }
    tot
}

fn prices(initial: i64) -> impl Iterator<Item = i64> {
    secrets(initial).map(|i| i % 10)
}

type Changes = (i64, i64, i64, i64);

fn first_prices(initial: i64) -> HashMap<Changes, i64> {
    let mut ret = HashMap::new();
    for (a1, a2, a3, a4, a5) in prices(initial).tuple_windows() {
        let changes = (a2 - a1, a3 - a2, a4 - a3, a5 - a4);
        ret.entry(changes).or_insert(a5);
    }
    ret
}

fn part2(initials: &Vec<i64>) -> i64 {
    let mut totals = HashMap::new();
    for i in initials {
        for (changes, bananas) in first_prices(*i) {
            *totals.entry(changes).or_default() += bananas;
        }
    }

    *totals.values().max().unwrap()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let initials = parse(&fname);
    println!("Part 1: {}", part1(&initials));
    println!("Part 2: {}", part2(&initials));
}
