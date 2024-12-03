use anyhow::Result;
use regex::RegexBuilder;
use std::env::args;
use std::fs::read_to_string;

fn calc(s: &str, with_enablement: bool) -> Result<i64> {
    let re = RegexBuilder::new(r" do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)")
        .ignore_whitespace(true)
        .build()?;
    let mut ret = 0;
    let mut enabled = true;

    for caps in re.captures_iter(s) {
        match caps.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a1: i64 = caps.get(1).unwrap().as_str().parse()?;
                let a2: i64 = caps.get(2).unwrap().as_str().parse()?;
                if enabled || !with_enablement {
                    ret += a1 * a2;
                }
            }
        }
    }

    Ok(ret)
}

fn main() {
    let fname = args().nth(1).unwrap();
    let s = read_to_string(fname).unwrap();
    println!("Part 1: {}", calc(&s, false).unwrap());
    println!("Part 2: {}", calc(&s, true).unwrap());
}
