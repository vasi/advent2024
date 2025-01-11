use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
enum BinOp {
    And,
    Or,
    Xor,
}

impl BinOp {
    fn parse(s: &str) -> Self {
        match s {
            "AND" => BinOp::And,
            "OR" => BinOp::Or,
            "XOR" => BinOp::Xor,
            _ => panic!("Invalid binary operation: {}", s),
        }
    }
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    op: BinOp,
    output: String,
}

impl Gate {
    fn deps(&self) -> Vec<&str> {
        vec![self.left.as_str(), self.right.as_str()]
    }

    fn is_ready(&self, inputs: &HashMap<String, bool>) -> bool {
        inputs.contains_key(&self.left) && inputs.contains_key(&self.right)
    }

    fn eval(&self, inputs: &HashMap<String, bool>) -> bool {
        let left = inputs.get(&self.left).unwrap();
        let right = inputs.get(&self.right).unwrap();
        match self.op {
            BinOp::And => left & right,
            BinOp::Or => left | right,
            BinOp::Xor => left ^ right,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl Circuit {
    fn parse(fname: &str) -> Self {
        let mut inputs = HashMap::new();
        let mut gates = vec![];

        let contents = read_to_string(fname).unwrap();
        let re_input = Regex::new(r"(\w+): ([01])").unwrap();
        let re_gate = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();

        for line in contents.lines() {
            if let Some(caps) = re_input.captures(line) {
                let val = match &caps[2] {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid input value: {}", &caps[2]),
                };
                inputs.insert(caps[1].to_string(), val);
            } else if let Some(caps) = re_gate.captures(line) {
                gates.push(Gate {
                    left: caps[1].to_string(),
                    right: caps[3].to_string(),
                    op: BinOp::parse(&caps[2]),
                    output: caps[4].to_string(),
                });
            }
        }

        Self { inputs, gates }
    }

    fn eval(&self) -> HashMap<String, bool> {
        let mut outputs = self.inputs.clone();

        let mut ready = Vec::new();
        let mut gates: HashMap<&str, &Gate> =
            self.gates.iter().map(|g| (g.output.as_str(), g)).collect();
        let mut deps: HashMap<&str, Vec<&Gate>> = HashMap::new();

        for gate in &self.gates {
            if gate.is_ready(&outputs) {
                ready.push(gate.output.as_str());
            }
            for dep in gate.deps() {
                deps.entry(dep).or_default().push(gate);
            }
        }

        while let Some(output) = ready.pop() {
            let gate = gates.remove(output).unwrap();
            let val = gate.eval(&outputs);
            outputs.insert(gate.output.clone(), val);

            if let Some(deps) = deps.remove(output) {
                for dep in deps {
                    if dep.is_ready(&outputs) {
                        ready.push(dep.output.as_str());
                    }
                }
            }
        }

        outputs
    }

    fn output(values: HashMap<String, bool>) -> i64 {
        let outputs = values
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted_by_key(|(k, _)| k.to_owned())
            .map(|(_, v)| v)
            .collect::<Vec<_>>();
        let mut ret = 0;
        for (i, v) in outputs.iter().enumerate() {
            if **v {
                ret += 1_i64 << i;
            }
        }
        ret
    }

    fn part1(&self) -> i64 {
        let eval = self.eval();
        Self::output(eval)
    }

    fn name(prefix: &str, idx: usize) -> String {
        format!("{}{:02}", prefix, idx)
    }

    fn set_input(&mut self, idx: usize, prefix: &str, n: i64) {
        self.inputs
            .insert(Self::name(prefix, idx), (n >> idx) & 1 != 0);
    }

    fn calculate(&mut self, x: i64, y: i64) -> i64 {
        for i in 0..=44 {
            self.set_input(i, "x", x);
            self.set_input(i, "y", y);
        }
        let results = self.eval();
        Self::output(results)
    }

    fn bits_set(n: i64) -> Vec<usize> {
        (0..=45).filter(|i| (n >> i) & 1 != 0).collect()
    }

    fn part2(&mut self) {
        for i in 0..=44 {
            let z = self.calculate(1 << i, 0);
            if z != (1 << i) {
                println!("x bit {} => {:?}", i, Self::bits_set(z));
            }

            let z = self.calculate(0, 1 << i);
            if z != (1 << i) {
                println!("y bit {} => {:?}", i, Self::bits_set(z));
            }

            let z = self.calculate(1 << i, 1 << i);
            if z != (1 << (i + 1)) {
                println!("both bit {} => {:?}", i, Self::bits_set(z));
            }

            if i != 44 {
                let z = self.calculate(3 << i, 3 << i);
                if z != (3 << (i + 1)) {
                    println!("3 bit {} => {:?}", i, Self::bits_set(z));
                }
            }
        }
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let mut circuit = Circuit::parse(&fname);
    // println!("Part 1: {}", circuit.part1());
    circuit.part2();
}
