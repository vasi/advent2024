use std::collections::HashMap;
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

fn next_val(st: i64) -> Vec<i64> {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    val: i64,
    rounds: usize,
}

impl Key {
    fn new(val: i64, rounds: usize) -> Self {
        Self { val, rounds }
    }
}

fn count_n(stones: &Stones, n: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&st| cache_count(&mut cache, &Key::new(st, n)))
        .sum()
}

fn cache_count(cache: &mut HashMap<Key, usize>, key: &Key) -> usize {
    if key.rounds == 0 {
        return 1;
    }

    if let Some(result) = cache.get(key) {
        return *result;
    }

    let mut tot = 0;
    for n in next_val(key.val) {
        let new_key = Key::new(n, key.rounds - 1);
        let count = cache_count(cache, &new_key);
        cache.insert(new_key, count);
        tot += count;
    }
    tot
}

fn main() {
    let fname = args().nth(1).unwrap();
    let stones = parse(&fname);
    println!("Part 1: {:?}", count_n(&stones, 25));
    println!("Part 1: {:?}", count_n(&stones, 75));
}
