use std::env::args;
use std::fmt;
use std::fs;

#[derive(Clone)]
struct Disk {
    bitmap: Vec<Option<i64>>,
}

impl Disk {
    fn parse(fname: &str) -> Self {
        let contents = fs::read_to_string(fname).unwrap();
        let mut bitmap = Vec::new();
        let mut next_id = 0;
        let mut next_is_block = true;
        for c in contents.chars() {
            let size = c.to_digit(10).unwrap();
            let n = if next_is_block {
                Some(next_id)
            } else {
                next_id += 1;
                None
            };
            for _ in 0..size {
                bitmap.push(n);
            }
            next_is_block = !next_is_block;
        }
        Self { bitmap }
    }

    fn compact1(&mut self) {
        let mut start = 0;
        let mut end = self.bitmap.len() - 1;
        while start < end {
            if self.bitmap.get(start).unwrap().is_some() {
                start += 1;
            } else if self.bitmap.get(end).unwrap().is_none() {
                end -= 1;
            } else {
                self.bitmap.swap(start, end);
            }
        }
    }

    fn score1(&self) -> i64 {
        let mut tot = 0;
        for (i, v) in self.bitmap.iter().enumerate() {
            if let Some(id) = v {
                tot += (i as i64) * id;
            }
        }
        tot
    }

    fn part1(&mut self) -> i64 {
        self.compact1();
        self.score1()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in self.bitmap.iter() {
            let c = e.map_or(".".to_owned(), |i| {
                let mut s = i.to_string();
                if s.len() > 1 {
                    s = "{".to_owned() + &s + "}";
                }
                s
            });
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let disk = Disk::parse(&fname);
    println!("{}", disk.clone().part1());
}
