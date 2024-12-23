use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

fn parse(fname: &str) -> Vec<String> {
    let contents = read_to_string(fname).unwrap();
    contents.lines().map(|s| s.to_owned()).collect()
}

type Dir = char;
type Loc = char;
type MovesSpec = HashMap<Loc, Vec<(Dir, Loc)>>;
type Moves = HashMap<Loc, HashMap<Dir, Loc>>;

fn opposite(dir: Dir) -> Dir {
    match dir {
        '>' => '<',
        'v' => '^',
        _ => unreachable!(),
    }
}

fn all_moves(unidir: MovesSpec) -> Moves {
    let mut ret = Moves::new();
    for (start, moves) in unidir {
        for (dir, target) in moves {
            ret.entry(start).or_default().insert(dir, target);
            ret.entry(target).or_default().insert(opposite(dir), start);
        }
    }
    ret
}

fn moves_numeric() -> Moves {
    all_moves(MovesSpec::from_iter([
        ('7', vec![('v', '4'), ('>', '8')]),
        ('8', vec![('v', '5'), ('>', '9')]),
        ('9', vec![('v', '6')]),
        ('4', vec![('v', '1'), ('>', '5')]),
        ('5', vec![('v', '2'), ('>', '6')]),
        ('6', vec![('v', '3')]),
        ('1', vec![/*       */ ('>', '2')]),
        ('2', vec![('v', '0'), ('>', '3')]),
        ('3', vec![('v', 'A')]),
        ('0', vec![/*       */ ('>', 'A')]),
    ]))
}

fn moves_dpad() -> Moves {
    all_moves(MovesSpec::from_iter([
        ('^', vec![('v', 'v'), ('>', 'A')]),
        ('A', vec![('v', '>')]),
        ('<', vec![/*       */ ('>', 'v')]),
        ('v', vec![/*       */ ('>', '>')]),
    ]))
}

type Cost = usize;
type AllCosts = HashMap<Loc, HashMap<Loc, Cost>>;
type State = (Loc, Loc); // (current, upper)

fn human_costs(moves: &Moves) -> AllCosts {
    moves
        .keys()
        .map(|k| {
            let targets = moves.keys().map(|t| (*t, 1)).collect();
            (*k, targets)
        })
        .collect()
}

fn successors(moves: &Moves, upper_costs: &AllCosts, state: &State) -> Vec<(State, Cost)> {
    let (cur, upper) = state;
    let mut nexts: Vec<(&Dir, &Loc)> = moves.get(&cur).unwrap().iter().collect_vec();
    if *upper != 'A' {
        nexts.push((&'A', cur))
    }
    nexts
        .iter()
        .map(|(dir, target)| {
            let cost = upper_costs.get(&upper).unwrap().get(dir).unwrap();
            ((**target, **dir), *cost)
        })
        .collect()
}

fn all_costs(moves: &Moves, upper_costs: &AllCosts) -> AllCosts {
    let mut ret = AllCosts::new();
    for start in moves.keys() {
        let pos = (*start, 'A');
        let mut costs = HashMap::new();
        costs.insert(*start, 1);

        let djikstra = dijkstra_all(&pos, |s| successors(moves, upper_costs, s));
        for ((target, dir), (_, cost)) in djikstra {
            if dir == 'A' {
                costs.insert(target, cost);
            }
        }
        ret.insert(*start, costs);
    }
    ret
}

fn seq_cost(costs: &AllCosts, seq: &Vec<Loc>) -> usize {
    let mut tot = 0;
    let mut pos = 'A';
    for l in seq {
        let cost = costs.get(&pos).unwrap().get(l).unwrap();
        tot += cost;
        pos = *l;
    }
    tot
}

#[allow(dead_code)]
fn print_costs(costs: &AllCosts) {
    let ks = costs.keys().sorted().collect_vec();
    print!("    ");
    for dest in &ks {
        print!("{:>4}", dest);
    }
    println!();
    for start in &ks {
        print!("{:>4}", start);
        for dest in &ks {
            print!("{:>4}", costs.get(start).unwrap().get(dest).unwrap());
        }
        println!();
    }
    println!()
}

fn leveln_costs(n: usize) -> AllCosts {
    let num_moves = moves_numeric();
    let dpad_moves = moves_dpad();

    let mut costs = human_costs(&dpad_moves);
    for _ in 0..(n - 2) {
        costs = all_costs(&dpad_moves, &costs);
    }
    costs = all_costs(&num_moves, &costs);
    costs
}

fn numeric_part(code: &str) -> usize {
    let non_digits = Regex::new(r"\D").unwrap();
    non_digits.replace_all(code, "").parse().unwrap()
}

fn part1(codes: &Vec<String>) -> usize {
    let costs = leveln_costs(4);
    codes
        .iter()
        .map(|code| {
            let numeric = numeric_part(&code);
            let seq = code.chars().collect_vec();
            let cost = seq_cost(&costs, &seq);
            cost * numeric
        })
        .sum()
}

fn main() {
    let fname = args().nth(1).unwrap();
    let codes = parse(&fname);
    println!("Part 1: {}", part1(&codes));
}
