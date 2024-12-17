use regex::Regex;
use std::env::args;
use std::fs;

#[derive(Debug)]
struct Program {
    registers: [i64; 3],
    ip: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl Program {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    fn parse(fname: &str) -> Self {
        let contents = fs::read_to_string(fname).unwrap();

        let re = Regex::new(
            r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d,]+)",
        )
        .unwrap();
        let caps = re.captures(&contents).unwrap();
        let registers = [
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
        ];
        let program = caps[4].split(',').map(|s| s.parse().unwrap()).collect();
        Self {
            registers,
            ip: 0,
            program,
            output: vec![],
        }
    }

    fn combo(&self, op: i64) -> i64 {
        match op {
            1..=3 => op,
            4..=6 => self.registers[op as usize - 4],
            _ => unreachable!(),
        }
    }

    fn xdv(&mut self, reg: usize, op: i64) {
        let combo = self.combo(op);
        let r = self.registers[Self::A] / 2i64.pow(combo as u32);
        self.registers[reg] = r;
    }

    fn run_one(&mut self) {
        let inst = self.program[self.ip];
        let op = self.program[self.ip + 1];
        let mut jmp = None;
        match inst {
            0 => self.xdv(Self::A, op),
            1 => self.registers[Self::B] = self.registers[Self::B] ^ op,
            2 => self.registers[Self::B] = self.combo(op) & 0x7,
            3 => {
                if self.registers[Self::A] != 0 {
                    jmp = Some(op);
                }
            }
            4 => self.registers[Self::B] = self.registers[Self::B] ^ self.registers[Self::C],
            5 => self.output.push(self.combo(op) & 0x7),
            6 => self.xdv(Self::B, op),
            7 => self.xdv(Self::C, op),
            _ => unreachable!(),
        }
        if let Some(target) = jmp {
            self.ip = target as usize;
        } else {
            self.ip += 2;
        }
    }

    fn run(&mut self) -> Vec<i64> {
        while self.ip < self.program.len() {
            self.run_one();
        }
        self.output.clone()
    }

    fn output_string(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let mut program = Program::parse(&fname);
    program.run();
    println!("Part 1: {}", program.output_string());
}
