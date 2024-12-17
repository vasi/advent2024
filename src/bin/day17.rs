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

    #[allow(dead_code)]
    fn state(&self) -> String {
        format!(
            "ip: {:02o}, A: {:9o}, B: {:9o}, C: {:9o}, output: {}",
            self.ip,
            self.registers[Self::A],
            self.registers[Self::B],
            self.registers[Self::C],
            self.output_string()
        )
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
        // println!("{}", self.state());
        while self.ip < self.program.len() {
            self.run_one();
            // println!("{}", self.state());
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

    fn combo_desc(&self, op: i64) -> String {
        match op {
            1..=3 => format!("{}", op),
            4..=6 => format!("{}", (b'A' + (op as u8 - 4)) as char),
            _ => unreachable!(),
        }
    }

    fn inst_desc(&self, inst: i64, op: i64) -> String {
        match inst {
            0 => format!("adv {}", self.combo_desc(op)),
            1 => format!("bxl {}", op),
            2 => format!("bst {}", self.combo_desc(op)),
            3 => format!("jnz {}", op),
            4 => "bxc".to_owned(),
            5 => format!("out {}", self.combo_desc(op)),
            6 => format!("bdv {}", self.combo_desc(op)),
            7 => format!("cdv {}", self.combo_desc(op)),
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    fn disassemble(&self) -> String {
        let mut ops = Vec::new();
        for ip in (0..self.program.len()).step_by(2) {
            let inst = self.program[ip];
            let op = self.program[ip + 1];
            ops.push(format!("{}: {}", ip, self.inst_desc(inst, op)));
        }
        ops.into_iter()
            .map(|o| o + "\n")
            .collect::<Vec<_>>()
            .join("")
    }

    // What could the next-to-last digit of A be?
    fn part2_candidates(&self, idx: i64, prev_a: i64) -> Vec<i64> {
        let mut ret = vec![];
        let target = self.program[15 - (idx as usize)];
        for b in 0..8 {
            if idx == 0 && b == 0 {
                continue; // can't start with zero
            }
            let a = (prev_a << 3) + b;
            let out = outval(a, b);
            if out == target {
                ret.push(b);
            }
        }
        // println!(
        //     "idx: {}, prev_a: {:o}, target: {:o}, candidates: {:?}",
        //     idx, prev_a, target, ret
        // );
        ret
    }

    fn part2_idx(&self, idx: i64, prev_a: i64) -> Option<i64> {
        if idx >= 16 {
            return Some(prev_a);
        }
        let candidates = self.part2_candidates(idx, prev_a);
        for b in candidates {
            let next_a = (prev_a << 3) + b;
            if let Some(r) = self.part2_idx(idx + 1, next_a) {
                return Some(r);
            }
        }
        None
    }

    // Specific to our program, not general!
    fn part2(&self) -> i64 {
        self.part2_idx(0, 0).unwrap()
    }
}

fn outval(a: i64, b: i64) -> i64 {
    let shift = b ^ 4;
    let c = (a >> shift) & 0x7;
    b ^ c
}

#[allow(dead_code)]
fn run_input(mut a: i64) {
    let mut ret = vec![];
    while a > 0 {
        let b = a & 0x7;
        ret.push(outval(a, b));
        a = a / 8;
    }
    println!("{:?}", ret);
}

fn main() {
    let fname = args().nth(1).unwrap();
    let mut program = Program::parse(&fname);
    // print!("{}", program.disassemble());
    program.run();
    println!("Part 1: {}", program.output_string());
    println!("Part 2: {}", program.part2());
}
