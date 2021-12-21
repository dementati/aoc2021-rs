use std::cmp;
use std::collections::HashMap;

use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let mut positions: Vec<i128> = input.split_whitespace()
        .tuples::<(&str, &str, &str, &str, &str)>()
        .map(|(_, _, _, _, x)| x.parse().unwrap())
        .collect();

    let mut scores = vec![0, 0];
    let mut roll_count = 0;
    let mut die = 0;
    loop {
        let mut offset = 0;

        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;

        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;
        
        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;
        
        positions[0] = (positions[0] - 1 + offset) % 10 + 1;
        scores[0] += positions[0];

        if cmp::max(scores[0], scores[1]) >= 1000 {
            break;
        }

        let mut offset = 0;

        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;
        
        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;
        
        die = (die + 1) % 100;
        roll_count += 1;
        offset += die;
        
        positions[1] = (positions[1] - 1 + offset) % 10 + 1;
        scores[1] += positions[1];

        if cmp::max(scores[0], scores[1]) >= 1000 {
            break;
        }
    }

    cmp::min(scores[0], scores[1]) * roll_count
}

fn star2(input: String) -> i128 {
    let positions: Vec<i128> = input.split_whitespace()
        .tuples::<(&str, &str, &str, &str, &str)>()
        .map(|(_, _, _, _, x)| x.parse().unwrap())
        .collect();

    let (wa, wb) = play((positions[0], positions[1]), 21);

    cmp::max(wa, wb)
}

type Args = ((i128, i128), (i128, i128), i128, bool, i128);
type Wins = (i128, i128);
type Cache = HashMap<Args, Wins>;

fn play(initial_positions: (i128, i128), win: i128) -> (i128, i128) {
    let mut cache: Cache = Cache::new();
    let mut stack: Vec<Args> = Vec::new();

    let initial_args = vec![
        (initial_positions, (0, 0), 1, true, 0),
        (initial_positions, (0, 0), 2, true, 0),
        (initial_positions, (0, 0), 3, true, 0),
    ];

    stack.extend(initial_args.iter());
    while !stack.is_empty() {
        let args = stack[stack.len() - 1];
        let ((mut pa, mut pb), (mut sa, mut sb), roll, turn, i) = args.clone();
        if turn && i == 2 {
            pa = (pa - 1 + roll) % 10 + 1;
            sa += pa;

            if sa >= win {
                cache.insert(args, (1, 0));
                stack.pop().unwrap();
                continue;
            } 
        } else if !turn && i == 2 {
            pb = (pb - 1 + roll) % 10 + 1;
            sb += pb;

            if sb >= win {
                cache.insert(args, (0, 1));
                stack.pop().unwrap();
                continue;
            } 
        }

        let new_args = vec![
            (
                (pa, pb), 
                (sa, sb), 
                if i == 2 {0} else {roll} + 1, 
                if i == 2 { !turn } else { turn }, 
                (i + 1) % 3
            ),
            (
                (pa, pb), 
                (sa, sb), 
                if i == 2 {0} else {roll} + 2, 
                if i == 2 { !turn } else { turn }, 
                (i + 1) % 3
            ),
            (
                (pa, pb), 
                (sa, sb), 
                if i == 2 {0} else {roll} + 3, 
                if i == 2 { !turn } else { turn }, 
                (i + 1) % 3
            ),
        ];

        if new_args.iter().all(|a| cache.contains_key(a)) {
            let wins = new_args.iter()
                .map(|a| cache[a])
                .reduce(|(wa1, wb1), (wa2, wb2)| (wa1 + wa2, wb1 + wb2))
                .unwrap();
            cache.insert(args, wins);
            stack.pop().unwrap();
        } else {
            stack.extend(new_args);
        }
    }

    initial_args.iter()
        .map(|a| cache[a])
        .reduce(|(wa1, wb1), (wa2, wb2)| (wa1 + wa2, wb1 + wb2))
        .unwrap()
}
