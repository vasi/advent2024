use std::env::args;
use std::fmt;
use std::fs;

type Bitmap = Vec<Option<i64>>;

#[derive(Clone)]
struct Gap {
    pos: usize,
    size: usize,
}

#[derive(Clone)]
struct File {
    #[allow(unused)]
    id: i64,
    pos: usize,
    size: usize,
}

#[derive(Clone)]
struct Disk {
    bitmap: Bitmap,
    files: Vec<File>,
    gaps: Vec<Gap>,
}

impl Disk {
    fn parse(fname: &str) -> Self {
        let contents = fs::read_to_string(fname).unwrap();
        let mut bitmap = Bitmap::new();
        let mut files = Vec::new();
        let mut gaps = Vec::new();
        let mut next_id = 0;
        let mut next_is_block = true;
        for c in contents.chars() {
            let size = c.to_digit(10).unwrap();
            let n = if next_is_block {
                files.push(File {
                    id: next_id,
                    pos: bitmap.len(),
                    size: size as usize,
                });
                Some(next_id)
            } else {
                gaps.push(Gap {
                    pos: bitmap.len(),
                    size: size as usize,
                });
                next_id += 1;
                None
            };
            for _ in 0..size {
                bitmap.push(n);
            }
            next_is_block = !next_is_block;
        }
        Self {
            bitmap,
            files,
            gaps,
        }
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

    fn move_file_in_bitmap(bitmap: &mut Bitmap, file: &File, gap: &mut Gap) {
        for i in 0..file.size {
            bitmap.swap(gap.pos + i, file.pos + i);
        }
        gap.pos += file.size;
        gap.size -= file.size;
    }

    fn compact2(&mut self) {
        for file in self.files.iter().rev() {
            if let Some(gap) = self
                .gaps
                .iter_mut()
                .find(|g| g.pos < file.pos && g.size >= file.size)
            {
                Self::move_file_in_bitmap(&mut self.bitmap, file, gap);
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

    fn part2(&mut self) -> i64 {
        self.compact2();
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
    println!("Part 1: {}", disk.clone().part1());
    println!("Part 2: {}", disk.clone().part2());
}
