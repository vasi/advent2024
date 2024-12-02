use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;

#[derive(Debug)]
struct Report(Vec<i64>);

impl Report {
    fn is_safe(&self) -> bool {
        let mut last: Option<i64> = None;
        let mut ldir = None;
        for i in &self.0 {
            if let Some(l) = last {
                let diff = i - l;
                if diff.abs() < 1 || diff.abs() > 3 {
                    return false;
                }
                let dir = diff.signum();
                if let Some(ld) = ldir {
                    if dir != ld {
                        return false;
                    }
                }
                ldir = Some(dir)
            }
            last = Some(*i)
        }
        true
    }

    fn is_dampened_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        for (pos, _) in self.0.iter().enumerate() {
            let r = self.0.iter().enumerate()
                .filter(|(p, _)| *p != pos)
                .map(|(_, i)| *i)
                .collect::<Vec<i64>>();
            if Report(r).is_safe() {
                return true;
            }
        }
        return false;
    }
}

fn parse(fname: String) -> Result<Vec<Report>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut reports = Vec::new();
    for rline in reader.lines() {
        let line = rline?;
        if line.is_empty() {
            break;
        }
        let levels: Result<Vec<i64>, _> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect();
        reports.push(Report(levels?));
    }
    Ok(reports)
}

fn part1(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn part2(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|r| r.is_dampened_safe()).count()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let reports = parse(fname).unwrap();

    let safe_count = part1(&reports);
    println!("Part 1: {}", safe_count);

    let dampened_safe_count = part2(&reports);
    println!("Part 2: {}", dampened_safe_count);
}
