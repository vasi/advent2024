use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

type Dir = char;
type Loc = char;
type Moves = HashMap<Loc, Vec<(Dir, Loc)>>;

fn opposite(dir: Dir) -> Dir {
    match dir {
        '>' => '<',
        'v' => '^',
        _ => unreachable!(),
    }
}

fn all_moves(unidir: Moves) -> Moves {
    let mut ret = Moves::new();
    for (start, moves) in unidir {
        for (dir, target) in moves {
            ret.entry(start).or_default().push((dir, target));
            ret.entry(target).or_default().push((opposite(dir), start));
        }
    }
    ret
}

fn moves_numeric() -> Moves {
    all_moves(Moves::from_iter([
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
    all_moves(Moves::from_iter([
        ('^', vec![('v', 'v'), ('>', 'A')]),
        ('A', vec![('v', '>')]),
        ('<', vec![/*       */ ('>', 'v')]),
        ('V', vec![/*       */ ('>', '>')]),
    ]))
}

fn main() {
    let num_moves = moves_numeric();
    let successors = |l: &Loc| {
        num_moves
            .get(l)
            .unwrap()
            .iter()
            .map(|(_, t)| (*t, 1))
            .collect_vec()
    };
    let costs = HashMap::<Loc, usize>::from_iter(
        dijkstra_all(&'A', successors)
            .iter()
            .map(|(l, (_, c))| (*l, *c)),
    );
    println!("{:?}", &costs);
}
