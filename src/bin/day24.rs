use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
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

type Gates = Vec<Gate>;
type Values = HashMap<String, bool>;

#[derive(Debug)]
struct Circuit<'a> {
    gates: &'a Vec<Gate>,
    swaps: HashMap<String, String>,
}

impl<'a> Circuit<'a> {
    fn parse(fname: &str) -> (Gates, Values) {
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
        (gates, inputs)
    }

    fn eval(&self, values: &mut Values) {
        let mut ready = Vec::new();
        let mut gates: HashMap<&str, &Gate> =
            self.gates.iter().map(|g| (g.output.as_str(), g)).collect();
        let mut deps: HashMap<&str, Vec<&Gate>> = HashMap::new();

        for gate in self.gates {
            if gate.is_ready(values) {
                ready.push(gate.output.as_str());
            }
            for dep in gate.deps() {
                deps.entry(dep).or_default().push(gate);
            }
        }

        while let Some(id) = ready.pop() {
            let gate = gates.remove(id).unwrap();
            let val = gate.eval(values);

            let real_output = self.swaps.get(id).cloned().unwrap_or(id.to_string());
            values.insert(real_output.clone(), val);

            if let Some(deps) = deps.remove(real_output.as_str()) {
                for dep in deps {
                    if dep.is_ready(values) {
                        ready.push(dep.output.as_str());
                    }
                }
            }
        }
    }

    fn output(values: &Values) -> i64 {
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

    fn part1(&self, values: &mut Values) -> i64 {
        self.eval(values);
        Self::output(&values)
    }

    fn name(prefix: &str, idx: usize) -> String {
        format!("{}{:02}", prefix, idx)
    }

    fn set_input(inputs: &mut Values, idx: usize, prefix: &str, n: i64) {
        inputs.insert(Self::name(prefix, idx), (n >> idx) & 1 != 0);
    }

    fn calculate(&self, x: i64, y: i64) -> i64 {
        let mut inputs = Values::new();
        for i in 0..=44 {
            Self::set_input(&mut inputs, i, "x", x);
            Self::set_input(&mut inputs, i, "y", y);
        }
        self.eval(&mut inputs);
        Self::output(&inputs)
    }

    #[allow(dead_code)]
    fn bits_set(n: i64) -> Vec<usize> {
        (0..=45).filter(|i| (n >> i) & 1 != 0).collect()
    }

    fn with_swap(&self, g1: &str, g2: &str) -> Self {
        let mut swaps = self.swaps.clone();
        swaps.insert(g1.to_string(), g2.to_string());
        swaps.insert(g2.to_string(), g1.to_string());
        Self {
            gates: self.gates,
            swaps,
        }
    }

    fn errors(&self) -> usize {
        let mut errs = 0;
        for i in 0..=44 {
            let z = self.calculate(1 << i, 0);
            if z != (1 << i) {
                errs += 1;
            }

            let z = self.calculate(0, 1 << i);
            if z != (1 << i) {
                errs += 1;
            }

            let z = self.calculate(1 << i, 1 << i);
            if z != (1 << (i + 1)) {
                errs += 1;
            }

            if i != 44 {
                let z = self.calculate(3 << i, 3 << i);
                if z != (3 << (i + 1)) {
                    errs += 1;
                }
            }
        }
        errs
    }

    fn names(&self) -> Vec<String> {
        self.gates.iter().map(|g| g.output.clone()).collect()
    }

    fn find_swap(&self, baseline: usize) -> Self {
        let mut names = self.names();
        names.sort();
        for (g1, g2) in names.iter().tuple_combinations() {
            println!("g1 = {}, g2 = {}", g1, g2);
            let changed = self.with_swap(g1, g2);
            let errs = changed.errors();
            if errs < baseline {
                println!("Found {}, {}. Errors = {}", g1, g2, errs);
                return self.with_swap(g1, g2);
            }
        }
        unreachable!()
    }

    fn part2(mut circuit: Circuit) -> String {
        let mut baseline = circuit.errors();
        while baseline > 0 {
            circuit = circuit.find_swap(baseline);
            baseline = circuit.errors();
        }

        circuit.swaps.keys().sorted().join(",")
    }
}

fn main() {
    let fname = args().nth(1).unwrap();
    let (gates, mut inputs) = Circuit::parse(&fname);
    let circuit = Circuit {
        gates: &gates,
        swaps: HashMap::new(),
    };

    println!("Part 1: {}", circuit.part1(&mut inputs));
    println!("Part 2: {}", Circuit::part2(circuit));
}
