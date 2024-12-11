use std::env::args;
use std::fs::read_to_string;

type Stones = Vec<i64>;

fn parse(fname: &str) -> Stones {
    let input = read_to_string(fname).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn digits(n: i64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn blink(stones: &Stones) -> Stones {
    stones
        .iter()
        .flat_map(|&st| {
            if st == 0 {
                vec![1]
            } else {
                let digits = digits(st);
                if digits % 2 == 0 {
                    vec![st / 10_i64.pow(digits / 2), st % 10_i64.pow(digits / 2)]
                } else {
                    vec![st * 2024]
                }
            }
        })
        .collect()
}

fn blink_n(stones: &Stones, n: usize) -> Stones {
    let mut stones = stones.clone();
    for _ in 0..n {
        stones = blink(&stones);
    }
    stones
}

fn main() {
    let fname = args().nth(1).unwrap();
    let stones = parse(&fname);
    println!("{:?}", blink_n(&stones, 25).len());
}
