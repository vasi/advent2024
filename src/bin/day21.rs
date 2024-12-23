use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

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
        ('V', vec![/*       */ ('>', '>')]),
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

fn main() {
    let num_moves = moves_numeric();
    let dpad_moves = moves_dpad();

    let mut costs = human_costs(&dpad_moves);
    costs = all_costs(&num_moves, &costs);

    let seq = "029A".chars().collect();
    println!("{}", seq_cost(&costs, &seq));
}
