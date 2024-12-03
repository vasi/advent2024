use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use std::env::args;
use std::fs::read_to_string;

fn part1(s: &str) -> Result<i64> {
    let re = RegexBuilder::new(r"mul\( (\d{1,3}) , (\d{1,3}) \)")
        .ignore_whitespace(true)
        .build()?;
    let mut ret = 0;

    for (_, caps) in re.captures_iter(s).map(|c| c.extract::<2>()) {
        let try_parsed: Result<Vec<i64>, _> = caps.iter().map(|c| c.parse()).collect();
        let parsed = try_parsed?;
        ret += parsed
            .into_iter()
            .reduce(|a, b| a * b)
            .ok_or(anyhow!("empty"))?;
    }

    Ok(ret)
}

fn main() {
    let fname = args().nth(1).unwrap();
    let s = read_to_string(fname).unwrap();
    println!("Part 1: {}", part1(&s).unwrap());
}
