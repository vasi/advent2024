use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::zip;
use anyhow::{anyhow, Result};

fn parse(fname: String) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for rline in reader.lines() {
        let line = rline?;
        if line.is_empty() {
            break;
        }
        let mut split = line.split_whitespace();
        v1.push(split.next().ok_or(anyhow!("missing value"))?.parse::<i32>()?);
        v2.push(split.next().ok_or(anyhow!("missing value"))?.parse::<i32>()?);
    }
    Ok((v1, v2))
}

fn calc1(mut v1: Vec<i32>, mut v2: Vec<i32>) -> Result<i32> {
    v1.sort();
    v2.sort();

    let mut r = 0;
    for (i1, i2) in zip(v1, v2) {
        r += (i1 - i2).abs();
    }
    Ok(r)
}

fn part1(fname: String) -> Result<()> {
    let (v1, v2) = parse(fname)?;
    let ret = calc1(v1, v2)?;
    println!("{:?}", ret);
    Ok(())
}

fn calc2(v1: Vec<i32>, v2: Vec<i32>) -> Result<i32> {
    let mut h = HashMap::new();
    for i2 in v2 {
        h.entry(i2).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut r = 0;
    for i1 in v1 {
        r += i1 * h.get(&i1).unwrap_or(&0);
    }
    Ok(r)
}

fn part2(fname: String) -> Result<()> {
    let (v1, v2) = parse(fname)?;
    let ret = calc2(v1, v2)?;
    println!("{:?}", ret);
    Ok(())
}

fn main() {
    part1(args().nth(1).unwrap()).unwrap();
    part2(args().nth(1).unwrap()).unwrap();
}
